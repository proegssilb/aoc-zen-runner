use std::{collections::HashMap, path::Path};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{parse_macro_input, ItemFn, Item, ItemMod, Ident, token::{Comma}, Type, ReturnType, parse::Parse, spanned::Spanned, ItemConst, Expr, Token};


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

struct AocCaseArgs {
    expected_p1: Expr,
    expected_p2: Option<Expr>,
}

impl Parse for AocCaseArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let p1: Expr = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![,]) {
            let _: Comma = input.parse()?;
            let p2: Expr = input.parse()?;
            Ok(AocCaseArgs { expected_p1: p1, expected_p2: Some(p2) })
        } else {
            if !input.is_empty() {
                Err(input.error("Expected: a single expression for just testing Part 1, or two expressions as two arguments if testing Part 1 and Part 2."))
            } else {
                Ok(AocCaseArgs { expected_p1: p1, expected_p2: None })
            }
        }
    }
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AocPart {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Eq)]
struct AocGeneratorArgs {
    display_slug: Ident,
}

#[derive(Debug, PartialEq, Eq)]
struct AocSolverArgs {
    problem_part: AocPart,
    display_slug: Ident,
}

#[derive(Debug, PartialEq, Eq)]
struct AocSolutionArgs {
    problem_part: AocPart,
    display_slug: Ident,
}

#[derive(Debug, PartialEq, Eq)]
struct AocMacroArgs {
    day_num: u32,
    output_type: Type,
}

impl Parse for AocPart {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "part1" => Ok(AocPart::Part1),
            "Part1" => Ok(AocPart::Part1),
            "p1" => Ok(AocPart::Part1),
            "P1" => Ok(AocPart::Part1),
            "part2" => Ok(AocPart::Part2),
            "Part2" => Ok(AocPart::Part2),
            "p2" => Ok(AocPart::Part2),
            "P2" => Ok(AocPart::Part2),
            _ => Err(input.error("Expected a Part 1 / Part 2 indicator, such as `part1` or `part2`.")),
        }
    }
}

impl Parse for AocGeneratorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let slug = input.parse::<Ident>()?;
        Ok(AocGeneratorArgs { display_slug: slug })
    }
}

impl Parse for AocSolverArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let part: AocPart = input.parse()?;
        input.parse::<Comma>()?;
        let slug: Ident = input.parse()?;
        Ok(AocSolverArgs { problem_part: part, display_slug: slug })
    }
}

impl Parse for AocSolutionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let part: AocPart = input.parse()?;
        input.parse::<Comma>()?;
        let slug: Ident = input.parse()?;
        Ok(AocSolutionArgs { problem_part: part, display_slug: slug })
    }
}

impl Parse for AocMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day_ident: Ident = input.parse()?;
        input.parse::<Comma>()?;
        let output_type: Type = input.parse()?;

        let day_part = day_ident.to_string();
        let day_part = day_part.strip_prefix("day").unwrap_or(&day_part);
        let day_part = day_part.strip_prefix("d").unwrap_or(&day_part);
        let day_num: u32 = day_part.parse().or_else(|a| {
            let msg = format!("Could not parse number from day indicator. Parsing error:\n{}", a);
            let e = syn::Error::new(day_ident.span(), msg);
            return Err(e);
        })?;

        if day_num < 1 || day_num > 25 {
            let e = syn::Error::new(day_ident.span(), "Day number is out of range of 1-25");
            return Err(e);
        }

        Ok(AocMacroArgs {output_type, day_num})
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AocGeneratorData<'a> {
    display_slug: Ident,
    gen_type: &'a Type,
    source: &'a ItemFn,
}

#[derive(Debug, PartialEq, Eq)]
struct AocSolverData<'a> {
    problem_part: AocPart,
    display_slug: Ident,
    input_type: &'a Type,
    source: &'a ItemFn,
}

#[derive(Debug, PartialEq, Eq)]
struct AocSolutionData<'a> {
    problem_part: AocPart,
    display_slug: Ident,
    source: &'a ItemFn,
}

impl<'a> AocGeneratorData<'a> {
    fn new(args: AocGeneratorArgs, source_fn: &'a ItemFn) -> syn::Result<AocGeneratorData<'a>> {
        let ReturnType::Type(_, ty_data) = &source_fn.sig.output else {
            let e = syn::Error::new(source_fn.sig.output.span(), "Generators must have a return type that can be passed to a solver function.");
            return Err(e);
        };
        Ok(AocGeneratorData { 
            display_slug: args.display_slug, 
            gen_type: ty_data.as_ref(), 
            source: &source_fn 
        })
    }
}

impl<'a> AocSolverData<'a> {
    fn new(args: AocSolverArgs, source_fn: &'a ItemFn) -> syn::Result<AocSolverData<'a>> {
        if source_fn.sig.inputs.len() != 1 {
            let e = syn::Error::new(source_fn.sig.inputs.span(), "Solvers must accept exactly one argument, the data from the generator. This argument may be a tuple, struct, or other type.");
            return Err(e);
        } else {
            let Some(solve_type) = source_fn.sig.inputs.first() else {
                panic!("This code should be unreachable.");
            };
            let syn::FnArg::Typed(solve_type) = solve_type else {
                let e = syn::Error::new(solve_type.span(), "Solvers cannot be methods which take a self param");
                return Err(e);
            };
            let solve_type = &solve_type.ty;
            return Ok(AocSolverData { 
                problem_part: args.problem_part, 
                display_slug: args.display_slug, 
                input_type: solve_type.as_ref(), 
                source: source_fn,
            });
        }
    }
}

impl<'a> AocSolutionData<'a> {
    fn new(args: AocSolutionArgs, source_fn: &'a ItemFn) -> AocSolutionData<'a> {
        AocSolutionData {
            problem_part: args.problem_part,
            display_slug: args.display_slug,
            source: source_fn,
        }
    }
}

struct AocSolutionsAggregation<'a> {
    solutions_p1: Vec<AocSolutionData<'a>>,
    solutions_p2: Vec<AocSolutionData<'a>>,
    generators: HashMap<&'a Type, Vec<AocGeneratorData<'a>>>,
    solvers_p1: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
    solvers_p2: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
}

impl<'a> AocSolutionsAggregation<'a> {
    fn new() -> Self {
        AocSolutionsAggregation { 
            solutions_p1: Vec::new(),
            solutions_p2: Vec::new(), 
            generators: HashMap::new(), 
            solvers_p1: HashMap::new(), 
            solvers_p2: HashMap::new(),
        }
    }

    fn p1_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p1.iter()
    }

    fn p2_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p2.iter()
    }

    fn p1_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p1.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter().flat_map(|g| {
                    self.solvers_p1.get(ty).unwrap().iter().map(move |s| (g, s))
                }).collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>().into_iter()
            }
        })
    }

    fn p2_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p2.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter().flat_map(|g| {
                    self.solvers_p2.get(ty).unwrap().iter().map(move |s| (g, s))
                }).collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>().into_iter()
            }
        })
    }
}

fn discover_mod_contents(module: &ItemMod) -> syn::Result<AocSolutionsAggregation> {
    let mut result = AocSolutionsAggregation::new();

    let Some((_, contents)) = &module.content else { return Ok(result); };
    for mod_item in contents.iter() {
        match mod_item {
            Item::Fn(fn_data) => {
                for attr in fn_data.attrs.iter() {
                    match attr.path.get_ident().map(|id| id.to_string()).as_deref() {
                        Some("generator") => {
                            let args = attr.parse_args::<AocGeneratorArgs>()?;
                            let data = AocGeneratorData::new(args, fn_data)?;
                            result.generators.entry(data.gen_type).or_default().push(data);
                        },
                        Some("solver") => {
                            let args = attr.parse_args::<AocSolverArgs>()?;
                            let data = AocSolverData::new(args, fn_data)?;
                            if data.problem_part == AocPart::Part1 {
                                result.solvers_p1.entry(data.input_type).or_default().push(data);
                            } else {
                                result.solvers_p2.entry(data.input_type).or_default().push(data);
                            }
                        },
                        Some("solution") => {
                            let args = attr.parse_args::<AocSolutionArgs>()?;
                            let data = AocSolutionData::new(args, fn_data);
                            if data.problem_part == AocPart::Part1 {
                                result.solutions_p1.push(data);
                            } else {
                                result.solutions_p2.push(data);
                            }
                        },
                        Some(_) => { continue; },
                        None => { continue; },
                    }
                }
            },
            _ => { continue; }
        }
    }

    Ok(result)
}

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

fn agg_to_solution_lists_mod(agg_result: &AocSolutionsAggregation, res_type: &Type, mod_name: &Ident ) -> proc_macro2::TokenStream {
    let p1_data: Vec<(&Ident, &Ident, Ident)> = gen_idents_from_solns("p1", agg_result.p1_composed_solns());

    let p1_fn_idents: Vec<&Ident> = p1_data.iter().map(|(_, _, f)| f).collect();
    let p1_gen_idents: Vec<&Ident> = p1_data.iter().map(|(g, _, _)| *g).collect();
    let p1_solver_idents: Vec<&Ident> = p1_data.iter().map(|(_, s, _)| *s).collect();

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

    if p1_data.len() == 0 && p2_data.len() == 0 {
        let err = agg_result.generators.keys().map(|k| {
            syn::Error::new(k.span(), "Could not locate solver for generator type.")
        }).fold(proc_macro2::TokenStream::new(), |mut acc, err| {
            let ts = err.to_compile_error();
            acc.extend(ts);
            acc
        });
        return err;
    }

    quote! {
        mod _gen_lists {
            use super::#mod_name::*;

            pub const P1_LABELS: [&str; #p1_len] = [ #(#p1_labels),* ];
            pub const P2_LABELS: [&str; #p2_len] = [ #(#p2_labels),* ];

            #(pub fn #p1_fn_idents(input: &str) -> #res_type { #p1_solver_idents(#p1_gen_idents(input)) })*
            #(pub fn #p2_fn_idents(input: &str) -> #res_type { #p2_solver_idents(#p2_gen_idents(input)) })*
            pub const P1_SOLUTIONS: [for<'r> fn(&'r str) -> #res_type; #p1_len] = [ #(#p1_impls),* ];
            pub const P2_SOLUTIONS: [for<'r> fn(&'r str) -> #res_type; #p2_len] = [ #(#p2_impls),* ];
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

    let mod_extension = agg_to_solution_lists_mod(&agg_result, &macro_args.output_type, mod_name);
    
    let mut item_ts = item.into_token_stream();

    item_ts.extend(mod_extension);
    item_ts.extend(gen_quick_microbench());
    item_ts.extend(gen_slow_microbench());
    item_ts.extend(gen_main(macro_args.day_num));

    item_ts.into()
}
