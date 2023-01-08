use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Lit, Meta};

#[derive(Debug)]
struct AssetField {
    pub ident: Ident,
    pub ty: AssetType,
}

// Asset type and path
#[derive(Debug)]
enum AssetType {
    // Vec<u8>
    File(String),
    Image(String),
    Sound(String),
    String(String),
    Texture(String),
    TtfFont(String),
}

#[derive(Default)]
struct AssetBuilder {
    pub ty: String,
    pub path: String,
}

impl AssetBuilder {
    pub fn build(self) -> AssetType {
        match self.ty.as_str() {
            "Sound" => AssetType::Sound(self.path),
            "Image" => AssetType::Image(self.path),
            "Texture" => AssetType::Texture(self.path),
            "TtfFont" => AssetType::TtfFont(self.path),
            "String" => AssetType::String(self.path),
            "File" => AssetType::File(self.path),
            _ => panic!(
                "Unknown asset type {}, must be in Sound, Image, Texture, TtfFont, String, File",
                self.ty
            ),
        }
    }
}

#[proc_macro_derive(AssetCollection, attributes(asset))]
pub fn my_derive_proc_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_asset_collection(ast).into()
}

fn impl_asset_collection(ast: DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut assets = vec![];

    if let Data::Struct(ds) = ast.data {
        if let Fields::Named(fields) = ds.fields {
            for field in fields.named.iter() {
                if let Some(asset) = parse_field(field) {
                    assets.push(asset);
                }
            }
        }
    }

    let start_coroutines = assets.iter().fold(quote!(), |token_stream, asset| {
        let ident = &asset.ident;
        let load_function = match &asset.ty {
            AssetType::File(path) => quote! {
                ::macroquad::file::load_file(#path)
            },
            AssetType::Image(path) => quote! {
                ::macroquad::texture::load_image(#path)
            },
            AssetType::Sound(path) => quote! {
                ::macroquad::audio::load_sound(#path)
            },
            AssetType::String(path) => quote! {
                ::macroquad::file::load_string(#path)
            },
            AssetType::Texture(path) => quote! {
                ::macroquad::texture::load_texture(#path)
            },
            AssetType::TtfFont(path) => quote! {
                ::macroquad::text::load_ttf_font(#path)
            },
        };
        quote! {
            #token_stream
            let #ident = coroutines::start_coroutine(#load_function);
        }
    });

    let retrieve_results = assets.iter().fold(quote!(), |token_stream, asset| {
        let ident = &asset.ident;

        quote! {
            #token_stream
            let #ident = loop {
                if #ident.is_done() {
                    break #ident.retrieve().unwrap().expect("Load asset #ident failed");
                }
                ::macroquad::prelude::next_frame().await;
            };
        }
    });

    let assign_fields = assets.iter().fold(quote!(), |token_stream, asset| {
        let ident = &asset.ident;

        quote! {
            #token_stream
            #ident,
        }
    });

    quote! {
        impl #name {
            pub async fn load() -> Self {
                #start_coroutines
                #retrieve_results
                Self {
                    #assign_fields
                }
            }
        }
    }
}

fn parse_field(field: &Field) -> Option<AssetField> {
    let field_ident = field.ident.clone().unwrap();
    for attr in field.attrs.iter() {
        if let Meta::List(ref asset_meta_list) = attr.parse_meta().unwrap() {
            if asset_meta_list.path.get_ident().unwrap() != "asset" {
                continue;
            }

            let mut asset_builder = AssetBuilder::default();
            for attribute in asset_meta_list.nested.iter() {
                if let syn::NestedMeta::Meta(meta) = attribute {
                    match meta {
                        Meta::Path(path) => {
                            let path_ident = path
                                .get_ident()
                                .expect("Failed to resolve nested meta")
                                .to_string();
                            asset_builder.ty = path_ident;
                        }
                        Meta::NameValue(kv) => {
                            let key = kv
                                .path
                                .get_ident()
                                .expect("Failed to resolve nested meta")
                                .to_string();
                            let value = if let Lit::Str(file_path) = &kv.lit {
                                file_path.value()
                            } else {
                                panic!("Failed to resolve kv pair");
                            };

                            if key != *"path" {
                                panic!("kv pair key must be path");
                            }

                            asset_builder.path = value;
                        }
                        _ => panic!("unexpected meta"),
                    }
                }
            }

            return Some(AssetField {
                ident: field_ident,
                ty: asset_builder.build(),
            });
        }
    }

    None
}
