use std::path::Path;

use aggregate::{AocSolutionsAggregation, discover_mod_contents};
use domain::{AocGeneratorData, AocSolverData};
use parser::caseargs::AocCaseArgs;
use parser::macroargs::AocMacroArgs;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::{ItemFn, ItemConst};
use syn::parse_macro_input;
use syn::{ItemMod, Ident};

mod parser;
mod partflag;
mod domain;
mod aggregate;

// Flag macros ------------------------------------------------------------
#[proc_macro_attribute]
pub fn generator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn solver(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn solution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn flag(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    println!("*** Flagged item in {}:\n{:#?}", file!(), &item);
    proc_macro::TokenStream::from(item.into_token_stream())
}

// Tests -------------------------------------------------------------------
#[proc_macro_attribute]
pub fn aoc_case(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AocCaseArgs);
    let exp_p1 = args.expected_p1;
    let p2 = args.expected_p2;
    let input = parse_macro_input!(item as ItemConst);
    let ty = input.ty.as_ref();
    let in_val = input.expr.as_ref();
    let slug_str: String = "aoc_test_".to_string() + input.ident.to_string().as_str();
    let slug = Ident::new(&slug_str, input.ident.span());

    if let Some(exp_p2) = p2 {
        quote! {
            #[test]
            fn #slug() {
                let expected_p1 = #exp_p1;
                let expected_p2 = #exp_p2;
                let input: #ty = #in_val;
        
                for p1 in super::_gen_lists::P1_SOLUTIONS {
                    assert_eq!(expected_p1, p1(input));
                }
                for p2 in super::_gen_lists::P2_SOLUTIONS {
                    assert_eq!(expected_p2, p2(input));
                }
            }
        }.into()
    } else {
        quote! {
            #[test]
            fn #slug() {
                let expected_p1 = #exp_p1;
                let input: #ty = #in_val;
        
                for p1 in _gen_lists::P1_SOLUTIONS {
                    assert_eq!(expected_p1, p1(input));
                }
            }
        }.into()
    }
    
}

// AOC --------------------------------------------------------------------

fn gen_idents_from_solns<'a>(part_indicator: &str, solns: impl Iterator<Item = (&'a AocGeneratorData<'a>, &'a AocSolverData<'a>)>) -> Vec<(&'a Ident, &'a Ident, Ident)> {
    solns.map(|(gen, sol)| {
        let g_ident = &gen.source.sig.ident;
        let g_slug = &gen.display_slug;
        let s_ident = &sol.source.sig.ident;
        let s_slug = &sol.display_slug;
        let f_ident = Ident::new(format!("f_{}_{}_{}", part_indicator, g_slug, s_slug).as_str(), Span::call_site());
        (g_ident, s_ident, f_ident)
    }).collect()
}

fn gen_composed_labels<'a>(solns: impl Iterator<Item = (&'a AocGeneratorData<'a>, &'a AocSolverData<'a>)>) -> Vec<String> {
    solns.map(|(gen, sol)| {
        let g_slug = &gen.display_slug.to_string();
        let s_slug = &sol.display_slug.to_string();
        let label = format!("{} / {}", g_slug, s_slug);
        label
    }).collect()
}

fn gen_solution_lists_mod(agg_result: &AocSolutionsAggregation, mod_name: &Ident ) -> proc_macro2::TokenStream {
    let p1_composed_data: Vec<(&Ident, &Ident, Ident)> = gen_idents_from_solns("p1", agg_result.p1_composed_solns());

    let p1_fn_idents: Vec<&Ident> = p1_composed_data.iter().map(|(_, _, f)| f).collect();
    let p1_gen_idents: Vec<&Ident> = p1_composed_data.iter().map(|(g, _, _)| *g).collect();
    let p1_solver_idents: Vec<&Ident> = p1_composed_data.iter().map(|(_, s, _)| *s).collect();

    let mut p1_labels = gen_composed_labels(agg_result.p1_composed_solns());
    let mut p1_impls = p1_fn_idents.clone();
    p1_impls.extend(agg_result.p1_user_solns().map(|sln| &sln.source.sig.ident));
    p1_labels.extend(agg_result.p1_user_solns().map(|sln| sln.display_slug.to_string()));
    let p1_len = p1_impls.len();

    let p2_data: Vec<(&Ident, &Ident, Ident)> = gen_idents_from_solns("p2", agg_result.p2_composed_solns());

    let p2_fn_idents: Vec<&Ident> = p2_data.iter().map(|(_, _, f)| f).collect();
    let p2_gen_idents: Vec<&Ident> = p2_data.iter().map(|(g, _, _)| *g).collect();
    let p2_solver_idents: Vec<&Ident> = p2_data.iter().map(|(_, s, _)| *s).collect();

    let mut p2_labels = gen_composed_labels(agg_result.p2_composed_solns());
    let mut p2_impls = p2_fn_idents.clone();
    p2_labels.extend(agg_result.p2_user_solns().map(|sln| sln.display_slug.to_string()));
    p2_impls.extend(agg_result.p2_user_solns().map(|sln| &sln.source.sig.ident));
    let p2_len = p2_impls.len();

    quote! {
        mod _gen_lists {
            use super::#mod_name::*;
            use std::fmt::Display;

            pub const P1_LABELS: [&str; #p1_len] = [ #(#p1_labels),* ];
            pub const P2_LABELS: [&str; #p2_len] = [ #(#p2_labels),* ];

            #(pub fn #p1_fn_idents(input: &str) -> impl Display { #p1_solver_idents(#p1_gen_idents(input)) })*
            #(pub fn #p2_fn_idents(input: &str) -> impl Display { #p2_solver_idents(#p2_gen_idents(input)) })*
            pub const P1_SOLUTIONS: [for<'r> fn(&'r str) -> impl Display; #p1_len] = [ #(#p1_impls),* ];
            pub const P2_SOLUTIONS: [for<'r> fn(&'r str) -> impl Display; #p2_len] = [ #(#p2_impls),* ];
        }
    }
}

fn gen_main(day_num: u32) -> proc_macro2::TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let inputs_path = "input/2022";
    let input_file = format!("{}.txt", day_num);
    let input_file_path = Path::new(&manifest_dir).join(inputs_path).join(input_file);
    let input_file_cow = input_file_path.to_string_lossy();
    let input_file = input_file_cow.as_ref();

    quote! {
        const AOC_RAW_INPUT: &str = include_str!(#input_file);

        #[cfg(not(test))]
        fn main() {
            println!("## AOC 2022, Day {} ----------", #day_num);
            let p1len = _gen_lists::P1_SOLUTIONS.len();
            let p2len = _gen_lists::P2_SOLUTIONS.len();
            if p1len > 0 {
                let solution_p1 = _gen_lists::P1_SOLUTIONS[0](AOC_RAW_INPUT);
                let label = _gen_lists::P1_LABELS[0];
                println!("Part 1, {} Solution: {}", label, solution_p1);
                if p1len > 1 {
                    println!("Checking alternative Part 1 solutions...");
                    for (idx, solver) in _gen_lists::P1_SOLUTIONS.iter().enumerate().skip(1) {
                        let solution = solver(AOC_RAW_INPUT);
                        if solution == solution_p1 {
                            print!("✅");
                        } else {
                            println!("\nSolver {} found {}", _gen_lists::P1_LABELS[idx], solution);
                        }
                    }
                    println!("");
                }
            }
            if p2len > 0 {
                let solution_p2 = _gen_lists::P2_SOLUTIONS[0](AOC_RAW_INPUT);
                let label = _gen_lists::P2_LABELS[0];
                println!("Part 2, {} Solution: {}", label, solution_p2);
                if p2len > 1 {
                    println!("Checking alternative Part 2 solutions...");
                    for (idx, solver) in _gen_lists::P2_SOLUTIONS.iter().enumerate().skip(1) {
                        let solution = solver(AOC_RAW_INPUT);
                        if solution == solution_p2 {
                            print!("✅");
                        } else {
                            println!("\nSolver {} found {}", _gen_lists::P2_LABELS[idx], solution);
                        }
                    }
                    println!("");
                }
            }
            
            println!(" ---- Quick Benches ----- ");
            bench_quick::run_benches();
        }
    }
}

fn gen_quick_microbench() -> proc_macro2::TokenStream {
    quote! {
        mod bench_quick {
            use std::time::Duration;
            use microbench as mb;

            pub fn run_benches() {
                let mb_opts = mb::Options::default().time(Duration::from_secs(1));

                for (idx, solver) in super::_gen_lists::P1_SOLUTIONS.iter().enumerate() {
                    let label = format!("Part 1 - {}", super::_gen_lists::P1_LABELS[idx]);
                    mb::bench(&mb_opts, &label, || solver(mb::retain(super::AOC_RAW_INPUT)))
                }
                for (idx, solver) in super::_gen_lists::P2_SOLUTIONS.iter().enumerate() {
                    let label = format!("Part 2 - {}", super::_gen_lists::P2_LABELS[idx]);
                    mb::bench(&mb_opts, &label, || solver( mb::retain(super::AOC_RAW_INPUT)))
                }
            }
        }
    }
}

fn gen_slow_microbench() -> proc_macro2::TokenStream {
    quote! {
        use pprof::criterion::{PProfProfiler, Output};
        use pprof::flamegraph::Options as FGOptions;
        use criterion::{Criterion, criterion_group, criterion_main, black_box};

        fn bench(c: &mut Criterion) {
            let mut group1 = c.benchmark_group("Part 1");
            for (idx, solver_fn) in _gen_lists::P1_SOLUTIONS.iter().enumerate() {
                let label = _gen_lists::P1_LABELS[idx];
                group1.bench_function(label, |b| b.iter(|| solver_fn(black_box(AOC_RAW_INPUT))));
            }
            group1.finish();
            let mut group2 = c.benchmark_group("Part 2");
            for (idx, solver_fn) in _gen_lists::P2_SOLUTIONS.iter().enumerate() {
                let label = _gen_lists::P2_LABELS[idx];
                group2.bench_function(label, |b| b.iter(|| solver_fn(black_box(AOC_RAW_INPUT))));
            }
            group2.finish();
        }

        criterion_group! {
            name = benches;
            config = Criterion::default()
                .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
                .with_output_color(true)
                .with_plots();
            targets = bench
        }

        // We need this call to happen only when benchmarking. This is the closest we can get.
        #[cfg(test)]
        criterion_main!(benches);
    }
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemMod);
    let mod_name = &item.ident;

    let macro_args = parse_macro_input!(args as AocMacroArgs);
    
    let agg_result = match discover_mod_contents(&item) {
        Ok(data) => data,
        Err(e) => {
            return e.into_compile_error().into();
        }
    };

    let mod_extension = gen_solution_lists_mod(&agg_result, mod_name);
    
    let mut item_ts = item.into_token_stream();

    item_ts.extend(mod_extension);
    item_ts.extend(gen_quick_microbench());
    item_ts.extend(gen_slow_microbench());
    item_ts.extend(gen_main(macro_args.day_num));

    item_ts.into()
}
