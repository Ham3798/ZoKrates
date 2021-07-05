//! Module containing structs and enums to represent imports.
//!
//! @file imports.rs
//! @author Thibaut Schaeffer <thibaut@schaeff.fr>
//! @date 2018

use crate::absy::*;
use crate::compile::compile_module;
use crate::compile::{CompileErrorInner, CompileErrors};
use crate::embed::FlatEmbed;
use crate::parser::Position;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use typed_arena::Arena;
use zokrates_common::Resolver;
use zokrates_field::{Bn128Field, Bw6_761Field, Field};

#[derive(PartialEq, Debug)]
pub struct Error {
    pos: Option<(Position, Position)>,
    message: String,
}

impl Error {
    pub fn new<T: Into<String>>(message: T) -> Error {
        Error {
            pos: None,
            message: message.into(),
        }
    }

    pub fn pos(&self) -> &Option<(Position, Position)> {
        &self.pos
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn with_pos(self, pos: Option<(Position, Position)>) -> Error {
        Error { pos, ..self }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self
            .pos
            .map(|p| format!("{}", p.0))
            .unwrap_or_else(|| "?".to_string());
        write!(f, "{}\n\t{}", location, self.message)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            pos: None,
            message: format!("I/O Error: {}", error),
        }
    }
}

pub struct Importer;

impl Importer {
    pub fn apply_imports<'ast, T: Field, E: Into<Error>>(
        destination: Module<'ast>,
        location: PathBuf,
        resolver: Option<&dyn Resolver<E>>,
        modules: &mut HashMap<OwnedModuleId, Module<'ast>>,
        arena: &'ast Arena<String>,
    ) -> Result<Module<'ast>, CompileErrors> {
        let symbols: Vec<_> = destination
            .symbols
            .into_iter()
            .map(|s| match s.value.symbol {
                Symbol::Here(SymbolDefinition::Import(import)) => {
                    Importer::resolve::<T, E>(import, &location, resolver, modules, arena)
                }
                _ => Ok(s),
            })
            .collect::<Result<_, _>>()?;

        Ok(Module::with_symbols(symbols))
    }

    fn resolve<'ast, T: Field, E: Into<Error>>(
        import: CanonicalImportNode<'ast>,
        location: &Path,
        resolver: Option<&dyn Resolver<E>>,
        modules: &mut HashMap<OwnedModuleId, Module<'ast>>,
        arena: &'ast Arena<String>,
    ) -> Result<SymbolDeclarationNode<'ast>, CompileErrors> {
        let pos = import.pos();
        let module_id = import.value.source;
        let symbol = import.value.id;

        let symbol_declaration = match module_id.to_str().unwrap() {
            "EMBED" => match symbol.id {
                #[cfg(feature = "bellman")]
                "sha256round" => {
                    if T::id() != Bn128Field::id() {
                        return Err(CompileErrorInner::ImportError(
                            Error::new(format!(
                                "Embed `sha256round` must be used with curve `{}`",
                                Bn128Field::name()
                            ))
                            .with_pos(Some(pos)),
                        )
                        .in_file(location)
                        .into());
                    } else {
                        SymbolDeclaration {
                            id: symbol.get_alias(),
                            symbol: Symbol::Flat(FlatEmbed::Sha256Round),
                        }
                    }
                }
                #[cfg(feature = "ark")]
                "snark_verify_bls12_377" => {
                    if T::id() != Bw6_761Field::id() {
                        return Err(CompileErrorInner::ImportError(
                            Error::new(format!(
                                "Embed `snark_verify_bls12_377` must be used with curve `{}`",
                                Bw6_761Field::name()
                            ))
                            .with_pos(Some(pos)),
                        )
                        .in_file(location)
                        .into());
                    } else {
                        SymbolDeclaration {
                            id: symbol.get_alias(),
                            symbol: Symbol::Flat(FlatEmbed::Verify),
                        }
                    }
                }
                "unpack" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::Unpack),
                },
                "u64_to_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U64ToBits),
                },
                "u32_to_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U32ToBits),
                },
                "u16_to_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U16ToBits),
                },
                "u8_to_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U8ToBits),
                },
                "u64_from_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U64FromBits),
                },
                "u32_from_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U32FromBits),
                },
                "u16_from_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U16FromBits),
                },
                "u8_from_bits" => SymbolDeclaration {
                    id: symbol.get_alias(),
                    symbol: Symbol::Flat(FlatEmbed::U8FromBits),
                },
                s => {
                    return Err(CompileErrorInner::ImportError(
                        Error::new(format!("Embed {} not found", s)).with_pos(Some(pos)),
                    )
                    .in_file(location)
                    .into());
                }
            },
            _ => match resolver {
                Some(res) => match res.resolve(location.to_path_buf(), module_id.to_path_buf()) {
                    Ok((source, new_location)) => {
                        let alias = symbol.alias.unwrap_or(
                            module_id
                                .file_stem()
                                .ok_or_else(|| {
                                    CompileErrors::from(
                                        CompileErrorInner::ImportError(Error::new(format!(
                                            "Could not determine alias for import {}",
                                            module_id.display()
                                        )))
                                        .in_file(location),
                                    )
                                })?
                                .to_str()
                                .unwrap(),
                        );

                        match modules.get(&new_location) {
                            Some(_) => {}
                            None => {
                                let source = arena.alloc(source);
                                let compiled = compile_module::<T, E>(
                                    source,
                                    new_location.clone(),
                                    resolver,
                                    modules,
                                    &arena,
                                )?;

                                assert!(modules.insert(new_location.clone(), compiled).is_none());
                            }
                        };

                        SymbolDeclaration {
                            id: &alias,
                            symbol: Symbol::There(
                                SymbolImport::with_id_in_module(symbol.id, new_location)
                                    .start_end(pos.0, pos.1),
                            ),
                        }
                    }
                    Err(err) => {
                        return Err(
                            CompileErrorInner::ImportError(err.into().with_pos(Some(pos)))
                                .in_file(location)
                                .into(),
                        );
                    }
                },
                None => {
                    return Err(CompileErrorInner::from(Error::new(
                        "Cannot resolve import without a resolver",
                    ))
                    .in_file(location)
                    .into());
                }
            },
        };

        Ok(symbol_declaration.start_end(pos.0, pos.1))
    }
}
