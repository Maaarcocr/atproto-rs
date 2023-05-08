extern crate proc_macro;
use std::io::Read;

use atproto_lexicon::{LexObject, LexUserType, LexiconDoc};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

struct Lexicon(LexiconDoc);

impl From<LexiconDoc> for Lexicon {
    fn from(doc: LexiconDoc) -> Self {
        Self(doc)
    }
}

fn obj_to_tokens(obj: &LexObject) -> TokenStream2 {
    match obj {
        LexObject::Object { description, required, nullable, properties } => {
            let fields = properties.and_then(|properties| {
                properties
                    .iter()
                    .map(|(name, prop)| {
                        let name = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
                        let ty = prop_to_tokens(prop);
                        quote! {
                            #name: #ty
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
            quote! {
                #[derive(Debug, Deserialize, Serialize)]
                #[serde(rename_all = "camelCase")]
                pub struct {
                    description: Option<String>,
                    required: Option<Vec<String>>,
                    nullable: Option<bool>,
                    #(#fields),*
                }
            }
        }
    }
}

fn def_to_tokens(name: &str, def: &LexUserType) -> TokenStream2 {
    let name = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
    match def {
        LexUserType::Record(record) => {}
        LexUserType::XrpcQuery(_) => todo!(),
        LexUserType::XrpcProcedure(_) => todo!(),
        LexUserType::XrpcSubscription(_) => todo!(),
        LexUserType::Blob(_) => todo!(),
        LexUserType::Array(_) => todo!(),
        LexUserType::Token(_) => todo!(),
        LexUserType::Object(_) => todo!(),
        LexUserType::Boolean(_) => todo!(),
        LexUserType::Integer(_) => todo!(),
        LexUserType::String(_) => todo!(),
        LexUserType::Bytes(_) => todo!(),
        LexUserType::CidLink(_) => todo!(),
        LexUserType::Unknown(_) => todo!(),
    }
}

impl ToTokens for Lexicon {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let id = self.0.id.replace(".", "_");
        let description = &self.0.description;
        let defs = self
            .0
            .defs
            .iter()
            .map(|(name, def)| def_to_tokens(name, def));
    }
}

#[proc_macro]
pub fn make_lexicons(item: TokenStream) -> TokenStream {
    // item is a path to a directory containing lexicons
    let dir_path = item.to_string();
    let dir_path = dir_path.trim_matches('"');
    let dir_path = std::path::Path::new(dir_path);
    let mut lexicons = Vec::new();
    for entry in walkdir::WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if file_path.extension().unwrap() != "json" {
                continue;
            }
            let mut file = std::fs::File::open(file_path).expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read file");
            let lexicon =
                atproto_lexicon::parse_lexicon_doc(&contents).expect("Unable to parse lexicon");
            lexicons.push(Lexicon::from(lexicon));
        }
    }

    quote!(
        #(#lexicons)*
    )
    .into()
}
