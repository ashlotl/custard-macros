#![feature(proc_macro_span)]

use std::{
	fs,
	path::{Component, Path},
};

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, LitStr};
use walkdir::WalkDir;

#[proc_macro]
pub fn attach_composition(_ast: TokenStream) -> TokenStream {
	(quote! {
		#[no_mangle]
		#[deny(warnings)]
		pub extern "C" fn __custard_composition__() -> custard_use::dylib_management::safe_library::core_library::CompositionFunctionReturn {
			let ret = custard_use::utils::files::get_maybe_const_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/composition.ron"), include_str!("composition.ron"));
			println!("fetched composition with status {}: {}", ret.1, ret.0);
			Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::from_rust(Ok(ret.0)))
		}

		#[allow(unused)]
		const __CUSTARD_MATCH_TYPE_COMPOSITION_FN__:custard_use::dylib_management::safe_library::core_library::CompositionFunctionType = __custard_composition__;
	})
	.into()
}

#[proc_macro]
pub fn attach_unloaded_datachunk_getter(_ast: TokenStream) -> TokenStream {
	let span = Span::call_site();
	let mut source_file_path = span.source_file().path();
	source_file_path.pop();

	let source_file_path_string = source_file_path.as_os_str().to_os_string().into_string().unwrap();

	let mut path = source_file_path.clone();
	path.push("user_data");

	let mut contents = vec![];
	for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
		if e.metadata().unwrap().is_file() {
			let mut clean_path = "".to_owned();
			let components = e.path().components().collect::<Vec<Component>>();
			for component_i in 2..components.len() {
				let component = components[component_i];
				clean_path.push_str(component.as_os_str().to_str().unwrap());
				if component_i != components.len() - 1 {
					clean_path.push('/');
				}
			}

			let universal = Path::new(clean_path.as_str()).clone();
			let universal = format!("{}", universal.display());

			contents.push((universal, fs::read_to_string(e.path()).unwrap()));
		}
	}

	let contents_len = contents.len();
	let mut contents_string = "".to_owned();
	for entry in contents {
		contents_string.push_str(format!("(r#\"{}\"#,r#\"{}\"#),", entry.0, entry.1).as_str());
	}

	// panic!(contents_string);

	let push_string = format!("const UNLOADED_DATACHUNKS: custard_use::utils::useful_statics::unloaded_static_array::UnloadedStaticArray<&'static str, &'static str, {}> = custard_use::utils::useful_statics::unloaded_static_array::UnloadedStaticArray {{ elems:[{}]}};", contents_len, contents_string);

	// panic!(push_string);

	let ret: TokenStream = format!(
		"{}\n{}",
		push_string,
		quote! {
			#[no_mangle]
			#[deny(warnings)]
			pub extern "C" fn __custard_unloaded_datachunk_contents__(path: Box<String>) -> Box<custard_use::dylib_management::safe_library::load_types::FFIResult<String, Box<dyn std::error::Error>>> {
				let path = path.as_str();
				let mut path_buf = std::path::PathBuf::new();
				path_buf.push(#source_file_path_string);
				path_buf.push(path);
				Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::from_rust(Ok(
					custard_use::utils::files::get_maybe_const_string(
						path_buf,
						match UNLOADED_DATACHUNKS.get(&path.clone()) {
							Ok(v) => v,
							Err(_e) => return Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::from_rust(Err(Box::new(custard_use::errors::load_errors::custard_unloaded_static_array_does_not_contain_element_error::CustardUnloadedStaticArrayDoesNotContainElementError { offending_key: path.to_owned() })))),
						},
					)
					.0,
				)))
			}
			#[allow(unused)]
		const __CUSTARD_MATCH_TYPE_UNLOADED_DATACHUNK_FN__: custard_use::dylib_management::safe_library::core_library::UnloadedDatachunkContentsFunctionType = __custard_unloaded_datachunk_contents__;
		}
	)
	.parse()
	.unwrap();
	ret.into()
}

#[proc_macro]
pub fn attach_unloaded_task_getter(_ast: TokenStream) -> TokenStream {
	let span = Span::call_site();
	let mut source_file_path = span.source_file().path();
	source_file_path.pop();

	let source_file_path_string = source_file_path.as_os_str().to_os_string().into_string().unwrap();

	let mut path = source_file_path.clone();
	path.push("user_data");

	let mut contents = vec![];
	for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
		if e.metadata().unwrap().is_file() {
			let mut clean_path = "".to_owned();
			let components = e.path().components().collect::<Vec<Component>>();
			for component_i in 2..components.len() {
				let component = components[component_i];
				clean_path.push_str(component.as_os_str().to_str().unwrap());
				if component_i != components.len() - 1 {
					clean_path.push('/');
				}
			}

			let universal = Path::new(clean_path.as_str()).clone();
			let universal = format!("{}", universal.display());

			contents.push((universal, fs::read_to_string(e.path()).unwrap()));
		}
	}

	let contents_len = contents.len();
	let mut contents_string = "".to_owned();
	for entry in contents {
		contents_string.push_str(format!("(r#\"{}\"#,r#\"{}\"#),", entry.0, entry.1).as_str());
	}

	// panic!(contents_string);

	let push_string = format!("const UNLOADED_TASKS: custard_use::utils::useful_statics::unloaded_static_array::UnloadedStaticArray<&'static str, &'static str, {}> = custard_use::utils::useful_statics::unloaded_static_array::UnloadedStaticArray {{ elems:[{}]}};", contents_len, contents_string);

	// panic!(push_string);

	let ret: TokenStream = format!(
		"{}\n{}",
		push_string,
		quote! {
			#[no_mangle]
			#[deny(warnings)]
			pub extern "C" fn __custard_unloaded_task_contents__(path: Box<String>) -> Box<custard_use::dylib_management::safe_library::load_types::FFIResult<String, Box<dyn std::error::Error>>> {

				let path = path.as_str();
				let mut path_buf = std::path::PathBuf::new();
				path_buf.push(#source_file_path_string);
				path_buf.push(path);

				let ret = Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::from_rust(Ok(
					custard_use::utils::files::get_maybe_const_string(
						path_buf,
						match UNLOADED_TASKS.get(&path.clone()) {
							Ok(v) => v,
							Err(_e) => {

								let dyn_error: Box<dyn std::error::Error> = Box::new(custard_use::errors::load_errors::custard_unloaded_static_array_does_not_contain_element_error::CustardUnloadedStaticArrayDoesNotContainElementError { offending_key: path.to_owned() });
								let ret = Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::from_rust(Err(dyn_error)));

								return ret;
							},
						},
					)
					.0,
				)));

				ret
			}

			#[allow(unused)]
		const __CUSTARD_MATCH_TYPE_UNLOADED_TASK_FN__: custard_use::dylib_management::safe_library::core_library::UnloadedTaskContentsFunctionType = __custard_unloaded_task_contents__;
		}
	)
	.parse()
	.unwrap();
	ret.into()
}

#[proc_macro]
pub fn attach_datachunk(ast: TokenStream) -> TokenStream {
	let data_name: Ident = parse_macro_input!(ast);
	let fn_name: Ident = format_ident!("__custard_datachunk__{}", data_name);
	(quote! {
		unsafe impl Send for #data_name {}
		unsafe impl Sync for #data_name {}

		#[no_mangle]
		#[allow(non_snake_case)]
		#[deny(warnings)]
		pub extern "C" fn #fn_name(
			from: Box<String>,
		) -> Box<custard_use::dylib_management::safe_library::load_types::FFIResult<Box<dyn custard_use::user_types::datachunk::Datachunk>, Box<dyn std::error::Error>>> {
			let created: Result<(#data_name), ron::Error> = ron::from_str(from.as_str());


			match created {
				Ok(v) => {
					return Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::Ok(Box::new(v)));
				}
				Err(e) => {
					return Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::Err(Box::new(e)));
				}
			}
		}

		#[allow(unused)]
		const __CUSTARD_MATCH_TYPE_DATACHUNK_LOAD_FN__: custard_use::dylib_management::safe_library::load_types::DatachunkLoadFn = #fn_name;
	})
	.into()
}

#[proc_macro]
pub fn attach_task(ast: TokenStream) -> TokenStream {
	let names: LitStr = parse_macro_input!(ast);
	let names_string = names.value();
	let split_names: Vec<&str> = names_string.split(":").collect();
	let task_impl = format_ident!("{}", split_names[0]);
	let task_data = format_ident!("{}", split_names[1]);

	let fn_name: Ident = format_ident!("__custard_task__{}", task_impl);
	(quote! {

		#[no_mangle]
		#[allow(non_snake_case)]
		#[deny(warnings)]
		pub extern "C" fn #fn_name(
			from: Box<String>,
		) -> Box<custard_use::dylib_management::safe_library::load_types::FFIResult<custard_use::user_types::task::Task, Box<dyn std::error::Error + Send + Sync>>> {
			let created: Result<(#task_data), ron::Error> = ron::from_str(from.as_str());


			match created {
				Ok(v) => {
					let task_data = std::sync::Arc::new(std::sync::Mutex::new(v));
					return Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::Ok(
						{
							custard_use::user_types::task::Task {
								task_data,
								task_impl: std::sync::Arc::new(std::sync::Mutex::new((#task_impl)())),
							}
						}
					));
				}
				Err(e) => {
					return Box::new(custard_use::dylib_management::safe_library::load_types::FFIResult::Err(Box::new(e)));
				}
			}
		}

		#[allow(unused)]
		const __CUSTARD_MATCH_TYPE_TASK_LOAD_FN__: custard_use::dylib_management::safe_library::load_types::TaskLoadFn = #fn_name;
	})
	.into()
}

#[proc_macro]
pub fn display_from_debug(ast: TokenStream) -> TokenStream {
	let type_name: Ident = parse_macro_input!(ast);
	(quote! {
		impl std::fmt::Display for #type_name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{:#?}", self)
			}
		}
	})
	.into()
}
