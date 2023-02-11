extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
	ItemImpl, ImplItem, Stmt, Meta, FnArg, Error,
    parse_macro_input, Data, DeriveInput, Fields, NestedMeta, Visibility,
    ImplItemMethod, Lit
};

/// A macro for deriving an implementation of `CheckRep`
///
/// The following usages of `#[rep]` are supported.
/// - `#[rep(assert_default)]`
/// - `#[rep(assert_true)]`
/// - `#[rep(assert_false)]`
/// - `#[rep(assert_eq = "---")]`
/// - `#[rep(assert_gt = 0.0)]`
/// - `#[rep(assert_lt = 100.0)]`
/// - `#[rep(assert_ge = 20)]`
/// - `#[rep(assert_le = 40)]`
/// - `#[rep(assert_with = "has_valid_id")]`
/// - `#[rep(check)]`
#[proc_macro_derive(CheckRep, attributes(rep))]
pub fn derive_check_rep(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let mut checks = vec![];
    let mut check_errors = vec![];
    let mut errors = vec![];
    let mut fields_to_recurse_on = vec![];
    let mut use_custom = false;

    if let Data::Struct(data_struct) = data {
        let fields = data_struct.fields;
        let mut fields_contents = None;
    	if let Fields::Named(fields_named) = fields {
            fields_contents = Some(fields_named.named);
        // } else if let Fields::Unnamed(fields_unnamed) = fields {
        //     fields_contents = Some(fields_unnamed.unnamed);
        } else {
            errors.push(Error::new(fields.span(), "expected named fields").to_compile_error());
        }

		for (_, field) in fields_contents.unwrap().iter().enumerate() {
			for attr in field.attrs.clone() {
				let maybe_meta = attr.parse_meta();

				if let Ok(meta) = maybe_meta {
					if let Meta::List(meta_list) = meta {
						if meta_list.path.is_ident("rep") {
							if meta_list.nested.len() == 1 {
								let nested = meta_list.nested.first().unwrap();

                                // #[rep] comes in 2 varieties
                                // 1. literals like #[rep(eq ="my_func")]
                                // 2. paths like #[rep(always_true)]
                                if let NestedMeta::Meta(nested_meta) = nested {
                                    let field_name = field.ident.clone().unwrap();//_or(Ident::new(&i.to_string(), Span::call_site()));
                                    let field_name_name = field.ident.clone().unwrap().to_string();
                                    let field_type = field.ty.clone();
                                    match nested_meta {
                                        Meta::Path(p) => {
                                            if p.is_ident("check") {
                                                fields_to_recurse_on.push(field_name);
                                            } else if p.is_ident("use_custom") {
                                                use_custom = true;
                                            } else if p.is_ident("assert_default") {
                                                checks.push(quote! {
                                                    {
                                                        let default: #field_type = Default::default();
                                                        self.#field_name == default
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be default, not {}", #field_name_name, self.#field_name)
                                                });
                                            } else if p.is_ident("assert_true") {
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be true", #field_name_name)
                                                });
                                            } else if p.is_ident("assert_false") {
                                                checks.push(quote! {
                                                    {
                                                        !self.#field_name
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be false", #field_name_name)
                                                });
                                            } else {
                                                errors.push(Error::new(p.span(), "unsupported representation invariant").to_compile_error());
                                            }
                                        }
                                        Meta::NameValue(v) => {
                                            if v.path.is_ident("assert_eq") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name == #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be {}, not {}", #field_name_name, #val, self.#field_name)
                                                });
                                            } else if v.path.is_ident("assert_ne") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name != #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must not be {}", #field_name_name, #val)
                                                });
                                            } else if v.path.is_ident("assert_gt") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name > #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be > {}, not {}", #field_name_name, #val, self.#field_name)
                                                });
                                            } else if v.path.is_ident("assert_lt") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name < #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be < {}, not {}", #field_name_name, #val, self.#field_name)
                                                });
                                            } else if v.path.is_ident("assert_ge") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name >= #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be >= {}, not {}", #field_name_name, #val, self.#field_name)
                                                });
                                            } else if v.path.is_ident("assert_le") {
                                                let val = v.lit.clone();
                                                checks.push(quote! {
                                                    {
                                                        self.#field_name <= #val
                                                    }
                                                });
                                                check_errors.push(quote! {
                                                    format!("self.{} must be <= {}, not {}", #field_name_name, #val, self.#field_name)
                                                });
                                            } else if v.path.is_ident("assert_with") {
                                                let val = v.lit.clone();
                                                if let Lit::Str(fn_name) = val.clone() {
                                                    let fn_to_call = fn_name.parse::<syn::Path>();
                                                    if let Ok(fn_to_call) = fn_to_call.clone() {
                                                        checks.push(quote! {
                                                        {
                                                            #fn_to_call ( self.#field_name )
                                                        }
                                                        });
                                                        check_errors.push(quote! {
                                                            format!("{}(self.{}) must be true when self.{} == {}", #fn_name, #field_name_name, #field_name_name, self.#field_name)
                                                        });   
                                                    } else {
                                                        errors.push(Error::new(val.span(), "assert_with can only be used with the name of a function to call").to_compile_error());
                                                    }
                                                } else {
                                                    errors.push(Error::new(val.span(), "assert_with can only be used with the name of a function to call").to_compile_error());    
                                                }
                                            } else {
                                                errors.push(Error::new(v.span(), "unsupported representation invariant").to_compile_error());
                                            }
                                        }
                                        _ => {
                                            errors.push(Error::new(nested_meta.span(), "unsupported representation invariant").to_compile_error());
                                        }

                                    }
                                } else {
                                    errors.push(Error::new(nested.span(), "invalid usage of #[rep]").to_compile_error());
                                }
							} else {
                                errors.push(Error::new(meta_list.span(), "expected just 1 item").to_compile_error());
							}
						}
					}
				}
			}
		}
    } else {
    	errors.push(Error::new(name.span(), "expected name of structure").to_compile_error());
    }

    let expanded = if errors.len() > 0 {
        quote! {
            impl rep::CheckRep for #name {
                fn is_correct(&self) -> Result<(), Vec<String>> {
                    Ok(())
                }

                fn check_rep(&self) {}
            }
            #(#errors)*
        }
    } else {
        if use_custom {
            quote! {
                impl rep::CheckRep for #name {
                    fn is_correct(&self) -> bool {
                        let mut is_correct = true;
                        #( is_correct = is_correct && #checks ; )*
                        #( is_correct = is_correct && self. #fields_to_recurse_on .is_correct() ; )*
                        is_correct
                    }
                    
                    fn correctness(&self) -> Result<(), Vec<String>> {
                        let mut c = vec![];
                        let mut is_error = false;
                        #( if ! #checks { c.push( #check_errors ); } )*
                        if c.len() > 0 {
                            is_error = true;
                        }
                        #( 
                            let recursed = self. #fields_to_recurse_on .correctness();
                            if let Err(mut errors) = recursed {
                                c.append(&mut errors);
                                is_error = true;
                            }
                        )*
                        let custom_check = self.c_correctness();
                        if let Err(mut errors) = custom_check {
                            c.append(&mut errors);
                            is_error = true;
                        }
                        if is_error {
                            Err(c)
                        } else {
                            Ok(())
                        }
                    }
                }
            }
        } else {
            quote! {
                impl rep::CheckRep for #name {
                    fn is_correct(&self) -> bool {
                        let mut is_correct = true;
                        #( is_correct = is_correct && #checks ; )*
                        #( is_correct = is_correct && self. #fields_to_recurse_on .is_correct() ; )*
                        is_correct
                    }

                    fn correctness(&self) -> Result<(), Vec<String>> {
                        let mut c = vec![];
                        let mut is_error = false;
                        #( if ! #checks { c.push( #check_errors ); } )*
                        if c.len() > 0 {
                            is_error = true;
                        }
                        #( 
                            let recursed = self. #fields_to_recurse_on .correctness();
                            if let Err(mut errors) = recursed {
                                c.append(&mut errors);
                                is_error = true;
                            }
                        )*
                        if is_error {
                            Err(c)
                        } else {
                            Ok(())
                        }
                    }
                }
            }            
        }
    };

    // hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// A macro that auto-inserts calls to `check_rep`
///
/// This macro can be applied to an `impl` block to inserts calls to `check_rep` only in methods that satisfy the following.
/// - Visibility is `pub`
/// - Parameters include `&mut self`
///
/// You may also apply it to a method in an `impl` block regardless of the method's signature.
#[proc_macro_attribute]
pub fn check_rep(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(mut impl_block) = syn::parse::<ItemImpl>(item.clone().into()) {
        let mut new_impl_block = impl_block.clone();
        new_impl_block.items = vec![];

        // loop through all items
        // see if the item is pub, accepts &mut self
        // if so, insert calls to check rep
    	for impl_item in &mut impl_block.items {
    		let mut new_impl_item = impl_item.clone();

    		if let ImplItem::Method(mut impl_item_method) = impl_item.clone() {
    			let mut new_impl_item_method = impl_item_method.clone();

                if let Visibility::Public(_) = new_impl_item_method.vis {
                    if new_impl_item_method.sig.inputs.iter().any(|input| {
                        if let FnArg::Receiver(receiver) = input {
                            receiver.mutability.is_some()
                        } else {
                            false
                        }
                    }) {
                        // insert calls to check rep at start and end of method
                        impl_item_method.block.stmts.insert(0, syn::parse::<Stmt>(quote! {
                            self.check_rep();
                        }.into()).unwrap());
                        impl_item_method.block.stmts.push(syn::parse::<Stmt>(quote! {
                            self.check_rep();
                        }.into()).unwrap());

                        new_impl_item_method.block.stmts = impl_item_method.block.stmts;
                        new_impl_item = ImplItem::Method(new_impl_item_method); 
                    }
                }
    		}

            new_impl_block.items.push(new_impl_item);
    	}

    	new_impl_block.to_token_stream().into()
    } else if let Ok(mut impl_item_method) = syn::parse::<ImplItemMethod>(item.clone().into()) {
        // insert calls to check rep at start and end of method
        impl_item_method.block.stmts.insert(0, syn::parse::<Stmt>(quote! {
            self.check_rep();
        }.into()).unwrap());
        impl_item_method.block.stmts.push(syn::parse::<Stmt>(quote! {
            self.check_rep();
        }.into()).unwrap());
        impl_item_method.to_token_stream().into()
    } else {
        let error = Error::new(Span::call_site(), "expected impl block or method").to_compile_error();

    	(quote! {
            #error
        }).into()
    }
}

/// A macro that inserts a call to `check_rep` at the start of given method
#[proc_macro_attribute]
pub fn require_rep(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(mut impl_item_method) = syn::parse::<ImplItemMethod>(item.clone().into()) {
        // insert calls to check rep at start and end of method
        impl_item_method.block.stmts.insert(0, syn::parse::<Stmt>(quote! {
            self.check_rep();
        }.into()).unwrap());
        impl_item_method.to_token_stream().into()
    } else {
        let error = Error::new(Span::call_site(), "expected method").to_compile_error();

        (quote! {
            #error
        }).into()
    }
}

/// A macro that inserts a call to `check_rep` at the start of given method
#[proc_macro_attribute]
pub fn ensure_rep(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(mut impl_item_method) = syn::parse::<ImplItemMethod>(item.clone().into()) {
        // insert calls to check rep at start and end of method
        impl_item_method.block.stmts.push(syn::parse::<Stmt>(quote! {
            self.check_rep();
        }.into()).unwrap());
        impl_item_method.to_token_stream().into()
    } else {
        let error = Error::new(Span::call_site(), "expected method").to_compile_error();

        (quote! {
            #error
        }).into()
    }
}