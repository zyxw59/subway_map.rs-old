use std::str::FromStr;
use ast::{Statement, Command, Definition, Expr, Scalar, Point, Line, Route, SIdent, PIdent, LIdent, RIdent, Ident};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Ident {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Statement, Command, Definition, Expr, Scalar, Point, Line, Route, SIdent, PIdent, LIdent, RIdent, Ident};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(&'input str),
        Term_22_5c_27_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2d_2d_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5c_5croutes_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7c_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_21_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(&'input str),
        Termr_23_22_40_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(&'input str),
        Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_7e_5c_5cw_2b_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(Expr),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Expr>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<Expr>),
        Nt_28_3cIdent_3e_20_22_2c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<Ident>),
        Nt_28_3cRIdent_3e_20_22_2c_22_29(RIdent),
        Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<RIdent>),
        Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<RIdent>),
        NtComma_3cExpr_3e(Vec<Expr>),
        NtComma_3cIdent_3e(Vec<Ident>),
        NtComma_3cRIdent_3e(Vec<RIdent>),
        NtCommand(Command),
        NtDefinition(Definition),
        NtExpr(Expr),
        NtExpr_3f(::std::option::Option<Expr>),
        NtIdent(Ident),
        NtIdent_3f(::std::option::Option<Ident>),
        NtLExpr(Line),
        NtLFactor(Line),
        NtLIdent(LIdent),
        NtLTerm(Line),
        NtNumber(f64),
        NtPExpr(Point),
        NtPFactor(Point),
        NtPIdent(PIdent),
        NtPTerm(Point),
        NtRExpr(Route),
        NtRIdent(RIdent),
        NtRIdent_3f(::std::option::Option<RIdent>),
        NtRTerm(Route),
        NtSArith(Scalar),
        NtSExpr(Scalar),
        NtSExpr_3f(::std::option::Option<Scalar>),
        NtSFactor(Scalar),
        NtSIdent(SIdent),
        NtSTerm(Scalar),
        NtStatement(Statement),
        NtString(String),
        Nt____Ident(Ident),
        Nt____Statement(Statement),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 7, 0, 8, 0,
        // State 1
        -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107, -107,
        // State 2
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 3
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 4
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 5
        -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53,
        // State 6
        -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68,
        // State 7
        -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -107,
        -43,
        -42,
        -41,
        -53,
        -68,
        -97,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""\'""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""--""###,
            r###""->""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""\\routes""###,
            r###""]""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###"r#"!\\w+"#"###,
            r###"r#"\"([^\"\\\\]|\\\\[\"\\\\])*\""#"###,
            r###"r#"@\\w+"#"###,
            r###"r#"[0-9]+(\\.[0-9]+)?"#"###,
            r###"r#"\\$\\w+"#"###,
            r###"r#"~\\w+"#"###,
        ];
        __ACTION[(__state * 31)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Ident<
        'input,
    >(
        input: &'input str,
    ) -> Result<Ident, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (6, _) if true => 0,
                (7, _) if true => 1,
                (8, _) if true => 2,
                (9, _) if true => 3,
                (10, _) if true => 4,
                (11, _) if true => 5,
                (12, _) if true => 6,
                (13, _) if true => 7,
                (14, _) if true => 8,
                (15, _) if true => 9,
                (16, _) if true => 10,
                (17, _) if true => 11,
                (18, _) if true => 12,
                (19, _) if true => 13,
                (20, _) if true => 14,
                (21, _) if true => 15,
                (22, _) if true => 16,
                (23, _) if true => 17,
                (24, _) if true => 18,
                (25, _) if true => 19,
                (26, _) if true => 20,
                (27, _) if true => 21,
                (28, _) if true => 22,
                (29, _) if true => 23,
                (30, _) if true => 24,
                (0, _) if true => 25,
                (1, _) if true => 26,
                (2, _) if true => 27,
                (3, _) if true => 28,
                (4, _) if true => 29,
                (5, _) if true => 30,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 31 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_2d_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_2d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (26, __tok0) => __Symbol::Term_22_5c_5croutes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (27, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (28, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (29, __tok0) => __Symbol::Term_22_7c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (30, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_21_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_40_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Termr_23_22_7e_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Ident,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(90);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action90::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(88);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action88::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(89);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action89::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(97);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action97::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(98);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action98::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Ident> ",") = Ident, "," => ActionFn(85);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action85::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Ident> ",")* =  => ActionFn(83);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action83::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(84);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action84::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Ident> ",")+ = Ident, "," => ActionFn(101);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action101::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(102);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action102::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<RIdent> ",") = RIdent, "," => ActionFn(80);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action80::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            12 => {
                // (<RIdent> ",")* =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<RIdent> ",")* = (<RIdent> ",")+ => ActionFn(79);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            14 => {
                // (<RIdent> ",")+ = RIdent, "," => ActionFn(105);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action105::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // (<RIdent> ",")+ = (<RIdent> ",")+, RIdent, "," => ActionFn(106);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtRIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action106::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            16 => {
                // Comma<Expr> = Expr => ActionFn(109);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action109::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            17 => {
                // Comma<Expr> =  => ActionFn(110);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action110::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            18 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(111);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action111::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            19 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(112);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action112::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            20 => {
                // Comma<Ident> = Ident => ActionFn(113);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action113::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            21 => {
                // Comma<Ident> =  => ActionFn(114);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action114::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<Ident> = (<Ident> ",")+, Ident => ActionFn(115);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action115::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<Ident> = (<Ident> ",")+ => ActionFn(116);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action116::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            24 => {
                // Comma<RIdent> = RIdent => ActionFn(117);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action117::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            25 => {
                // Comma<RIdent> =  => ActionFn(118);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action118::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            26 => {
                // Comma<RIdent> = (<RIdent> ",")+, RIdent => ActionFn(119);
                let __sym1 = __pop_NtRIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action119::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            27 => {
                // Comma<RIdent> = (<RIdent> ",")+ => ActionFn(120);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action120::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            28 => {
                // Command = "\\routes", "{", Comma<RIdent>, "}", String => ActionFn(5);
                let __sym4 = __pop_NtString(__symbols);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_NtComma_3cRIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_Term_22_5c_5croutes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                12
            }
            29 => {
                // Definition = SIdent, "=", SExpr => ActionFn(6);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            30 => {
                // Definition = PIdent, "=", PExpr => ActionFn(7);
                let __sym2 = __pop_NtPExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            31 => {
                // Definition = LIdent, "=", LExpr => ActionFn(8);
                let __sym2 = __pop_NtLExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            32 => {
                // Definition = RIdent, "=", RExpr => ActionFn(9);
                let __sym2 = __pop_NtRExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            33 => {
                // Definition = SIdent, "[", Comma<Ident>, "]", "=", SExpr => ActionFn(10);
                let __sym5 = __pop_NtSExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            34 => {
                // Definition = PIdent, "[", Comma<Ident>, "]", "=", PExpr => ActionFn(11);
                let __sym5 = __pop_NtPExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            35 => {
                // Definition = LIdent, "[", Comma<Ident>, "]", "=", LExpr => ActionFn(12);
                let __sym5 = __pop_NtLExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            36 => {
                // Expr = SExpr => ActionFn(13);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            37 => {
                // Expr = PExpr => ActionFn(14);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            38 => {
                // Expr = LExpr => ActionFn(15);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            39 => {
                // Expr? = Expr => ActionFn(86);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action86::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                15
            }
            40 => {
                // Expr? =  => ActionFn(87);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action87::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                15
            }
            41 => {
                // Ident = SIdent => ActionFn(62);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action62::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            42 => {
                // Ident = PIdent => ActionFn(63);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            43 => {
                // Ident = LIdent => ActionFn(64);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            44 => {
                // Ident? = Ident => ActionFn(81);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                17
            }
            45 => {
                // Ident? =  => ActionFn(82);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action82::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                17
            }
            46 => {
                // LExpr = LExpr, "+", PFactor => ActionFn(46);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            47 => {
                // LExpr = LExpr, "-", PFactor => ActionFn(47);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action47::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            48 => {
                // LExpr = LExpr, "|", PFactor => ActionFn(48);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            49 => {
                // LExpr = LExpr, ":", PFactor => ActionFn(49);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            50 => {
                // LExpr = LExpr, "\'", SFactor => ActionFn(50);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action50::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            51 => {
                // LExpr = LFactor => ActionFn(51);
                let __sym0 = __pop_NtLFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            52 => {
                // LFactor = LTerm => ActionFn(52);
                let __sym0 = __pop_NtLTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLFactor(__nt), __end));
                19
            }
            53 => {
                // LIdent = r#"!\\w+"# => ActionFn(67);
                let __sym0 = __pop_Termr_23_22_21_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLIdent(__nt), __end));
                20
            }
            54 => {
                // LTerm = PTerm, "--", PTerm => ActionFn(53);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2d_2d_22(__symbols);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            55 => {
                // LTerm = PTerm, "->", PTerm => ActionFn(54);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            56 => {
                // LTerm = "(", LExpr, ")" => ActionFn(55);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtLExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            57 => {
                // LTerm = LIdent => ActionFn(56);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            58 => {
                // LTerm = LIdent, "[", Comma<Expr>, "]" => ActionFn(57);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action57::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            59 => {
                // Number = r#"[0-9]+(\\.[0-9]+)?"# => ActionFn(69);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                22
            }
            60 => {
                // PExpr = PExpr, "+", PFactor => ActionFn(33);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            61 => {
                // PExpr = PExpr, "-", PFactor => ActionFn(34);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            62 => {
                // PExpr = PFactor => ActionFn(35);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            63 => {
                // PFactor = SFactor, "*", PTerm => ActionFn(36);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            64 => {
                // PFactor = PFactor, "*", STerm => ActionFn(37);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            65 => {
                // PFactor = PFactor, "/", STerm => ActionFn(38);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            66 => {
                // PFactor = LTerm, "&", LTerm => ActionFn(39);
                let __sym2 = __pop_NtLTerm(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_NtLTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            67 => {
                // PFactor = PTerm => ActionFn(40);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            68 => {
                // PIdent = r#"@\\w+"# => ActionFn(66);
                let __sym0 = __pop_Termr_23_22_40_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPIdent(__nt), __end));
                25
            }
            69 => {
                // PTerm = "(", SExpr, ",", SExpr, ")" => ActionFn(41);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtSExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtSExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            70 => {
                // PTerm = "-", PTerm => ActionFn(42);
                let __sym1 = __pop_NtPTerm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            71 => {
                // PTerm = "(", PExpr, ")" => ActionFn(43);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtPExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            72 => {
                // PTerm = PIdent => ActionFn(44);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            73 => {
                // PTerm = PIdent, "[", Comma<Expr>, "]" => ActionFn(45);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            74 => {
                // RExpr = RExpr, "\'", SExpr, ",", PExpr => ActionFn(121);
                let __sym4 = __pop_NtPExpr(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action121::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            75 => {
                // RExpr = RExpr, "\'", ",", PExpr => ActionFn(122);
                let __sym3 = __pop_NtPExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action122::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            76 => {
                // RExpr = RExpr, ",", RTerm => ActionFn(59);
                let __sym2 = __pop_NtRTerm(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action59::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            77 => {
                // RExpr = RTerm => ActionFn(60);
                let __sym0 = __pop_NtRTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            78 => {
                // RIdent = r#"~\\w+"# => ActionFn(68);
                let __sym0 = __pop_Termr_23_22_7e_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRIdent(__nt), __end));
                28
            }
            79 => {
                // RIdent? = RIdent => ActionFn(76);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action76::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRIdent_3f(__nt), __end));
                29
            }
            80 => {
                // RIdent? =  => ActionFn(77);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action77::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtRIdent_3f(__nt), __end));
                29
            }
            81 => {
                // RTerm = PExpr, "\'", SExpr, ",", PExpr => ActionFn(123);
                let __sym4 = __pop_NtPExpr(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action123::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtRTerm(__nt), __end));
                30
            }
            82 => {
                // RTerm = PExpr, "\'", ",", PExpr => ActionFn(124);
                let __sym3 = __pop_NtPExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action124::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRTerm(__nt), __end));
                30
            }
            83 => {
                // SArith = SArith, "+", SFactor => ActionFn(22);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            84 => {
                // SArith = SArith, "-", SFactor => ActionFn(23);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            85 => {
                // SArith = SFactor => ActionFn(24);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            86 => {
                // SExpr = SExpr, "<", SArith => ActionFn(16);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            87 => {
                // SExpr = SExpr, "<=", SArith => ActionFn(17);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3c_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            88 => {
                // SExpr = SExpr, ">", SArith => ActionFn(18);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3e_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            89 => {
                // SExpr = SExpr, ">=", SArith => ActionFn(19);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3e_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            90 => {
                // SExpr = SExpr, "==", SArith => ActionFn(20);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3d_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            91 => {
                // SExpr = SArith => ActionFn(21);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            92 => {
                // SExpr? = SExpr => ActionFn(71);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpr_3f(__nt), __end));
                33
            }
            93 => {
                // SExpr? =  => ActionFn(72);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action72::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpr_3f(__nt), __end));
                33
            }
            94 => {
                // SFactor = SFactor, "*", STerm => ActionFn(25);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            95 => {
                // SFactor = SFactor, "/", STerm => ActionFn(26);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            96 => {
                // SFactor = STerm => ActionFn(27);
                let __sym0 = __pop_NtSTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            97 => {
                // SIdent = r#"\\$\\w+"# => ActionFn(65);
                let __sym0 = __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSIdent(__nt), __end));
                35
            }
            98 => {
                // STerm = "-", STerm => ActionFn(28);
                let __sym1 = __pop_NtSTerm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            99 => {
                // STerm = Number => ActionFn(29);
                let __sym0 = __pop_NtNumber(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            100 => {
                // STerm = "(", SExpr, ")" => ActionFn(30);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            101 => {
                // STerm = SIdent => ActionFn(31);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            102 => {
                // STerm = SIdent, "[", Comma<Expr>, "]" => ActionFn(32);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            103 => {
                // Statement = Definition, ";" => ActionFn(2);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            104 => {
                // Statement = Command, ";" => ActionFn(3);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            105 => {
                // Statement =  => ActionFn(4);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action4::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            106 => {
                // String = r#"\"([^\"\\\\]|\\\\[\"\\\\])*\""# => ActionFn(70);
                let __sym0 = __pop_Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtString(__nt), __end));
                38
            }
            107 => {
                // __Ident = Ident => ActionFn(1);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            108 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_NtStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Statement(__nt), __end));
                40
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 41 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5croutes_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5croutes_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_21_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_21_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_40_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_40_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_7e_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_7e_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cRIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cRIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDefinition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Definition, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDefinition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumber<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumber(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Route, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Route, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSArith<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSArith(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Scalar>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtString<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtString(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ident<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ident(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Statement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Ident::parse_Ident;

mod __parse__Statement {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Statement, Command, Definition, Expr, Scalar, Point, Line, Route, SIdent, PIdent, LIdent, RIdent, Ident};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(&'input str),
        Term_22_5c_27_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2d_2d_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5c_5croutes_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7c_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_21_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(&'input str),
        Termr_23_22_40_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(&'input str),
        Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(&'input str),
        Termr_23_22_7e_5c_5cw_2b_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(Expr),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Expr>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<Expr>),
        Nt_28_3cIdent_3e_20_22_2c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<Ident>),
        Nt_28_3cRIdent_3e_20_22_2c_22_29(RIdent),
        Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<RIdent>),
        Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<RIdent>),
        NtComma_3cExpr_3e(Vec<Expr>),
        NtComma_3cIdent_3e(Vec<Ident>),
        NtComma_3cRIdent_3e(Vec<RIdent>),
        NtCommand(Command),
        NtDefinition(Definition),
        NtExpr(Expr),
        NtExpr_3f(::std::option::Option<Expr>),
        NtIdent(Ident),
        NtIdent_3f(::std::option::Option<Ident>),
        NtLExpr(Line),
        NtLFactor(Line),
        NtLIdent(LIdent),
        NtLTerm(Line),
        NtNumber(f64),
        NtPExpr(Point),
        NtPFactor(Point),
        NtPIdent(PIdent),
        NtPTerm(Point),
        NtRExpr(Route),
        NtRIdent(RIdent),
        NtRIdent_3f(::std::option::Option<RIdent>),
        NtRTerm(Route),
        NtSArith(Scalar),
        NtSExpr(Scalar),
        NtSExpr_3f(::std::option::Option<Scalar>),
        NtSFactor(Scalar),
        NtSIdent(SIdent),
        NtSTerm(Scalar),
        NtStatement(Statement),
        NtString(String),
        Nt____Ident(Ident),
        Nt____Statement(Statement),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 10, 0, 11, 0, 12, 13,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108, -108,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53,
        // State 10
        -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68, -68,
        // State 11
        -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97, -97,
        // State 12
        -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78, -78,
        // State 13
        -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104, -104,
        // State 14
        -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103, -103,
        // State 15
        0, 0, 30, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 10, 0, 11, 0, 12, 0,
        // State 17
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 10, 0, 11, 0, 12, 0,
        // State 19
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 20
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 10, 0, 11, 0, 12, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 13,
        // State 23
        0, 62, 0, 0, 0, 63, 0, 64, 0, 0, 0, 65, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51,
        // State 25
        -57, -57, 0, -57, -57, -57, -57, -57, 0, 0, -57, -57, -57, 0, 0, 0, 0, 0, 0, 67, 0, -57, 0, -57, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52,
        // State 27
        -72, -72, 0, -72, -72, -72, -72, -72, -72, -72, -72, -72, -72, 0, 0, 0, 0, 0, 0, 68, 0, -72, 0, -72, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 69, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 30
        0, 0, 77, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 10, 0, 11, 0, 12, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 35
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 36
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 37
        81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99, -99,
        // State 39
        0, 0, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -62, 0, -62, 84, -62, -62, -62, 0, 0, 85, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -67, 0, -67, -67, -67, -67, -67, 69, 70, -67, -67, -67, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, -67, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -101, 0, -101, -101, -101, -101, -101, 0, 0, -101, -101, -101, -101, -101, 0, -101, -101, -101, 88, 0, -101, 0, -101, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96, -96,
        // State 45
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 46
        0, 0, 91, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 48, 12, 0,
        // State 47
        -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 93, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 94, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77, -77,
        // State 52
        0, 0, 0, -91, 0, 96, -91, 97, 0, 0, 0, 0, -91, -91, -91, 0, -91, -91, -91, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, -85, 103, -85, -85, -85, 0, 0, 87, 0, -85, -85, -85, 0, -85, -85, -85, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 56
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 13,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 62
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 63
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 64
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 65
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 66
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 67
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 68
        0, 0, 77, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 69
        0, 0, 77, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 70
        0, 62, 0, 123, 0, 63, 0, 64, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        81, -52, 0, -52, 0, -52, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 124, 0, 82, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 125, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, -85, 86, -85, -85, -85, 0, 0, 87, 0, 0, -85, -85, 0, -85, -85, -85, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70,
        // State 76
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 80
        0, 0, 30, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 0, 0, 0,
        // State 81
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 82
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 83
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 84
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 85
        0, 0, 91, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 48, 12, 0,
        // State 86
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 87
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 88
        0, 0, 0, 137, 0, 0, 125, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98, -98,
        // State 90
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 56, 0, 0, 0, 140, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 93
        0, 0, 56, 0, 0, 0, 142, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 94
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 95
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 96
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 97
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 98
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 99
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 100
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 101
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 102
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 103
        0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 154, 0, 0, 0, 0,
        // State 107
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 108
        0, -50, 0, -50, 103, -50, -50, -50, 0, 0, 87, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, -46, 0, -46, 84, -46, -46, -46, 0, 0, 85, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, -47, 0, -47, 84, -47, -47, -47, 0, 0, 85, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, -49, 0, -49, 84, -49, -49, -49, 0, 0, 85, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, -48, 0, -48, 84, -48, -48, -48, 0, 0, 85, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 62, 0, 0, 0, 63, -38, 64, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 66, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 82, -37, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54,
        // State 121
        -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55,
        // State 122
        -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56,
        // State 123
        -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71, -71,
        // State 124
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 125
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 126
        0, 0, 30, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 0, 0, 0,
        // State 127
        -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66, -66,
        // State 128
        0, -60, 0, -60, 84, -60, -60, -60, 0, 0, 85, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, -61, 0, -61, 84, -61, -61, -61, 0, 0, 85, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64,
        // State 131
        -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65,
        // State 132
        -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63, -63,
        // State 133
        -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94, -94,
        // State 134
        -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95, -95,
        // State 135
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100,
        // State 137
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 138
        0, 0, 0, 0, 0, 0, 163, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 142
        -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76, -76,
        // State 143
        0, 0, 0, -83, 103, -83, -83, -83, 0, 0, 87, 0, -83, -83, -83, 0, -83, -83, -83, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, -84, 103, -84, -84, -84, 0, 0, 87, 0, -84, -84, -84, 0, -84, -84, -84, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, -86, 0, 96, -86, 97, 0, 0, 0, 0, -86, -86, -86, 0, -86, -86, -86, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, 0, 0, -87, 0, 96, -87, 97, 0, 0, 0, 0, -87, -87, -87, 0, -87, -87, -87, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, 0, 0, -90, 0, 96, -90, 97, 0, 0, 0, 0, -90, -90, -90, 0, -90, -90, -90, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        0, 0, 0, -88, 0, 96, -88, 97, 0, 0, 0, 0, -88, -88, -88, 0, -88, -88, -88, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, -89, 0, 96, -89, 97, 0, 0, 0, 0, -89, -89, -89, 0, -89, -89, -89, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 12, 0,
        // State 151
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 152
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 153
        -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106, -106,
        // State 154
        0, 0, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58,
        // State 156
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 157
        -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73, -73,
        // State 158
        0, 0, 0, 169, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 159
        0, 62, 0, 0, 0, 63, 0, 64, 0, 0, 0, 65, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0,
        // State 160
        -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102, -102,
        // State 161
        0, 0, 0, 0, 0, 82, 0, 83, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 162
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 163
        0, -82, 0, 0, 0, 82, -82, 83, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 164
        0, 0, 46, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 11, 48, 12, 0,
        // State 165
        0, -75, 0, 0, 0, 82, -75, 83, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 166
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 98, 99, 0, 100, 101, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 167
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 168
        -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69, -69,
        // State 169
        0, -81, 0, 0, 0, 82, -81, 83, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 170
        0, -74, 0, 0, 0, 82, -74, 83, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -105,
        0,
        0,
        0,
        0,
        0,
        0,
        -108,
        0,
        -53,
        -68,
        -97,
        -78,
        -104,
        -103,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -51,
        0,
        -52,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -43,
        -42,
        -41,
        0,
        -99,
        0,
        0,
        0,
        0,
        0,
        -96,
        0,
        0,
        -59,
        0,
        0,
        0,
        -77,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -70,
        0,
        0,
        0,
        -9,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -98,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -14,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -54,
        -55,
        -56,
        -71,
        0,
        -10,
        0,
        -66,
        0,
        0,
        -64,
        -65,
        -63,
        -94,
        -95,
        0,
        -100,
        0,
        0,
        0,
        0,
        0,
        -76,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -15,
        -28,
        -106,
        0,
        -58,
        -4,
        -73,
        0,
        0,
        -102,
        0,
        0,
        0,
        0,
        0,
        0,
        -5,
        -69,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 5, 0, 0, 6, 0, 0, 0, 0, 0, 0, 7, 0, 8, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 26, 27, 0, 0, 0, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 40, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 50, 41, 28, 42, 51, 0, 0, 52, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 54, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 34, 0, 0, 0, 35, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 25, 26, 72, 39, 73, 41, 28, 42, 0, 0, 0, 0, 53, 74, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 35, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 25, 26, 72, 39, 73, 41, 28, 42, 0, 0, 0, 0, 53, 89, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 28, 76, 0, 0, 0, 0, 0, 0, 0, 0, 44, 90, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 104, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 90, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 44, 45, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 110, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 111, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 112, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 113, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 66
        0, 0, 114, 0, 0, 0, 0, 0, 0, 115, 0, 0, 0, 0, 116, 0, 0, 0, 117, 25, 26, 72, 39, 118, 41, 28, 42, 0, 0, 0, 0, 53, 119, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 67
        0, 0, 114, 0, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0, 116, 0, 0, 0, 117, 25, 26, 72, 39, 118, 41, 28, 42, 0, 0, 0, 0, 53, 119, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 73, 41, 28, 42, 0, 0, 0, 0, 53, 74, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 128, 0, 0, 0, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 129, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 0, 130, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 131, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 132, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 28, 133, 0, 0, 0, 0, 0, 0, 0, 0, 44, 134, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 135, 0, 0, 0, 0,
        // State 87
        0, 0, 114, 0, 0, 0, 0, 0, 0, 136, 0, 0, 0, 0, 116, 0, 0, 0, 117, 25, 26, 72, 39, 118, 41, 28, 42, 0, 0, 0, 0, 53, 119, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 73, 41, 28, 42, 0, 0, 0, 0, 53, 89, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 139, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 141, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 50, 41, 28, 42, 0, 0, 0, 143, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 144, 44, 45, 0, 0, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 44, 45, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 146, 0, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 148, 0, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 149, 0, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 150, 0, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 134, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155, 0, 0, 0, 117, 25, 26, 72, 39, 118, 41, 28, 42, 0, 0, 0, 0, 53, 119, 0, 75, 44, 45, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 159, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 25, 26, 27, 0, 0, 0, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 132
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 135
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 137
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 162, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 138
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 164, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 166, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 142
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 53, 167, 0, 55, 44, 45, 0, 0, 0, 0,
        // State 151
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 153
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 157
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 159
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 160
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 161
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 162
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 170, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 163
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 164
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 38, 39, 171, 41, 28, 42, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 0, 0, 0,
        // State 165
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 166
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 167
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 168
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 169
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 170
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""\'""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""--""###,
            r###""->""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""\\routes""###,
            r###""]""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###"r#"!\\w+"#"###,
            r###"r#"\"([^\"\\\\]|\\\\[\"\\\\])*\""#"###,
            r###"r#"@\\w+"#"###,
            r###"r#"[0-9]+(\\.[0-9]+)?"#"###,
            r###"r#"\\$\\w+"#"###,
            r###"r#"~\\w+"#"###,
        ];
        __ACTION[(__state * 31)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Statement<
        'input,
    >(
        input: &'input str,
    ) -> Result<Statement, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (6, _) if true => 0,
                (7, _) if true => 1,
                (8, _) if true => 2,
                (9, _) if true => 3,
                (10, _) if true => 4,
                (11, _) if true => 5,
                (12, _) if true => 6,
                (13, _) if true => 7,
                (14, _) if true => 8,
                (15, _) if true => 9,
                (16, _) if true => 10,
                (17, _) if true => 11,
                (18, _) if true => 12,
                (19, _) if true => 13,
                (20, _) if true => 14,
                (21, _) if true => 15,
                (22, _) if true => 16,
                (23, _) if true => 17,
                (24, _) if true => 18,
                (25, _) if true => 19,
                (26, _) if true => 20,
                (27, _) if true => 21,
                (28, _) if true => 22,
                (29, _) if true => 23,
                (30, _) if true => 24,
                (0, _) if true => 25,
                (1, _) if true => 26,
                (2, _) if true => 27,
                (3, _) if true => 28,
                (4, _) if true => 29,
                (5, _) if true => 30,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 31 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_2d_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_2d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (26, __tok0) => __Symbol::Term_22_5c_5croutes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (27, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (28, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (29, __tok0) => __Symbol::Term_22_7c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (30, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_21_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_40_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Termr_23_22_7e_5c_5cw_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Statement,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(90);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action90::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(88);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action88::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(89);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action89::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(97);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action97::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(98);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action98::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Ident> ",") = Ident, "," => ActionFn(85);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action85::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Ident> ",")* =  => ActionFn(83);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action83::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(84);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action84::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Ident> ",")+ = Ident, "," => ActionFn(101);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action101::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(102);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action102::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<RIdent> ",") = RIdent, "," => ActionFn(80);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action80::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            12 => {
                // (<RIdent> ",")* =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<RIdent> ",")* = (<RIdent> ",")+ => ActionFn(79);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            14 => {
                // (<RIdent> ",")+ = RIdent, "," => ActionFn(105);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action105::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // (<RIdent> ",")+ = (<RIdent> ",")+, RIdent, "," => ActionFn(106);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtRIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action106::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            16 => {
                // Comma<Expr> = Expr => ActionFn(109);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action109::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            17 => {
                // Comma<Expr> =  => ActionFn(110);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action110::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            18 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(111);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action111::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            19 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(112);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action112::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            20 => {
                // Comma<Ident> = Ident => ActionFn(113);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action113::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            21 => {
                // Comma<Ident> =  => ActionFn(114);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action114::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<Ident> = (<Ident> ",")+, Ident => ActionFn(115);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action115::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<Ident> = (<Ident> ",")+ => ActionFn(116);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action116::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            24 => {
                // Comma<RIdent> = RIdent => ActionFn(117);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action117::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            25 => {
                // Comma<RIdent> =  => ActionFn(118);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action118::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            26 => {
                // Comma<RIdent> = (<RIdent> ",")+, RIdent => ActionFn(119);
                let __sym1 = __pop_NtRIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action119::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            27 => {
                // Comma<RIdent> = (<RIdent> ",")+ => ActionFn(120);
                let __sym0 = __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action120::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cRIdent_3e(__nt), __end));
                11
            }
            28 => {
                // Command = "\\routes", "{", Comma<RIdent>, "}", String => ActionFn(5);
                let __sym4 = __pop_NtString(__symbols);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_NtComma_3cRIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_Term_22_5c_5croutes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                12
            }
            29 => {
                // Definition = SIdent, "=", SExpr => ActionFn(6);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            30 => {
                // Definition = PIdent, "=", PExpr => ActionFn(7);
                let __sym2 = __pop_NtPExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            31 => {
                // Definition = LIdent, "=", LExpr => ActionFn(8);
                let __sym2 = __pop_NtLExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            32 => {
                // Definition = RIdent, "=", RExpr => ActionFn(9);
                let __sym2 = __pop_NtRExpr(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            33 => {
                // Definition = SIdent, "[", Comma<Ident>, "]", "=", SExpr => ActionFn(10);
                let __sym5 = __pop_NtSExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            34 => {
                // Definition = PIdent, "[", Comma<Ident>, "]", "=", PExpr => ActionFn(11);
                let __sym5 = __pop_NtPExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            35 => {
                // Definition = LIdent, "[", Comma<Ident>, "]", "=", LExpr => ActionFn(12);
                let __sym5 = __pop_NtLExpr(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtDefinition(__nt), __end));
                13
            }
            36 => {
                // Expr = SExpr => ActionFn(13);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            37 => {
                // Expr = PExpr => ActionFn(14);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            38 => {
                // Expr = LExpr => ActionFn(15);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                14
            }
            39 => {
                // Expr? = Expr => ActionFn(86);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action86::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                15
            }
            40 => {
                // Expr? =  => ActionFn(87);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action87::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                15
            }
            41 => {
                // Ident = SIdent => ActionFn(62);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action62::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            42 => {
                // Ident = PIdent => ActionFn(63);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            43 => {
                // Ident = LIdent => ActionFn(64);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                16
            }
            44 => {
                // Ident? = Ident => ActionFn(81);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                17
            }
            45 => {
                // Ident? =  => ActionFn(82);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action82::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                17
            }
            46 => {
                // LExpr = LExpr, "+", PFactor => ActionFn(46);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            47 => {
                // LExpr = LExpr, "-", PFactor => ActionFn(47);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action47::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            48 => {
                // LExpr = LExpr, "|", PFactor => ActionFn(48);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            49 => {
                // LExpr = LExpr, ":", PFactor => ActionFn(49);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            50 => {
                // LExpr = LExpr, "\'", SFactor => ActionFn(50);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtLExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action50::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            51 => {
                // LExpr = LFactor => ActionFn(51);
                let __sym0 = __pop_NtLFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLExpr(__nt), __end));
                18
            }
            52 => {
                // LFactor = LTerm => ActionFn(52);
                let __sym0 = __pop_NtLTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLFactor(__nt), __end));
                19
            }
            53 => {
                // LIdent = r#"!\\w+"# => ActionFn(67);
                let __sym0 = __pop_Termr_23_22_21_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLIdent(__nt), __end));
                20
            }
            54 => {
                // LTerm = PTerm, "--", PTerm => ActionFn(53);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2d_2d_22(__symbols);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            55 => {
                // LTerm = PTerm, "->", PTerm => ActionFn(54);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            56 => {
                // LTerm = "(", LExpr, ")" => ActionFn(55);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtLExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            57 => {
                // LTerm = LIdent => ActionFn(56);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            58 => {
                // LTerm = LIdent, "[", Comma<Expr>, "]" => ActionFn(57);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtLIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action57::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtLTerm(__nt), __end));
                21
            }
            59 => {
                // Number = r#"[0-9]+(\\.[0-9]+)?"# => ActionFn(69);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action69::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                22
            }
            60 => {
                // PExpr = PExpr, "+", PFactor => ActionFn(33);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            61 => {
                // PExpr = PExpr, "-", PFactor => ActionFn(34);
                let __sym2 = __pop_NtPFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            62 => {
                // PExpr = PFactor => ActionFn(35);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPExpr(__nt), __end));
                23
            }
            63 => {
                // PFactor = SFactor, "*", PTerm => ActionFn(36);
                let __sym2 = __pop_NtPTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            64 => {
                // PFactor = PFactor, "*", STerm => ActionFn(37);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            65 => {
                // PFactor = PFactor, "/", STerm => ActionFn(38);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtPFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            66 => {
                // PFactor = LTerm, "&", LTerm => ActionFn(39);
                let __sym2 = __pop_NtLTerm(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_NtLTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            67 => {
                // PFactor = PTerm => ActionFn(40);
                let __sym0 = __pop_NtPTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPFactor(__nt), __end));
                24
            }
            68 => {
                // PIdent = r#"@\\w+"# => ActionFn(66);
                let __sym0 = __pop_Termr_23_22_40_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPIdent(__nt), __end));
                25
            }
            69 => {
                // PTerm = "(", SExpr, ",", SExpr, ")" => ActionFn(41);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtSExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtSExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            70 => {
                // PTerm = "-", PTerm => ActionFn(42);
                let __sym1 = __pop_NtPTerm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            71 => {
                // PTerm = "(", PExpr, ")" => ActionFn(43);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtPExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            72 => {
                // PTerm = PIdent => ActionFn(44);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            73 => {
                // PTerm = PIdent, "[", Comma<Expr>, "]" => ActionFn(45);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtPIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtPTerm(__nt), __end));
                26
            }
            74 => {
                // RExpr = RExpr, "\'", SExpr, ",", PExpr => ActionFn(121);
                let __sym4 = __pop_NtPExpr(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action121::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            75 => {
                // RExpr = RExpr, "\'", ",", PExpr => ActionFn(122);
                let __sym3 = __pop_NtPExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action122::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            76 => {
                // RExpr = RExpr, ",", RTerm => ActionFn(59);
                let __sym2 = __pop_NtRTerm(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtRExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action59::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            77 => {
                // RExpr = RTerm => ActionFn(60);
                let __sym0 = __pop_NtRTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRExpr(__nt), __end));
                27
            }
            78 => {
                // RIdent = r#"~\\w+"# => ActionFn(68);
                let __sym0 = __pop_Termr_23_22_7e_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRIdent(__nt), __end));
                28
            }
            79 => {
                // RIdent? = RIdent => ActionFn(76);
                let __sym0 = __pop_NtRIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action76::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRIdent_3f(__nt), __end));
                29
            }
            80 => {
                // RIdent? =  => ActionFn(77);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action77::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtRIdent_3f(__nt), __end));
                29
            }
            81 => {
                // RTerm = PExpr, "\'", SExpr, ",", PExpr => ActionFn(123);
                let __sym4 = __pop_NtPExpr(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtSExpr(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action123::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtRTerm(__nt), __end));
                30
            }
            82 => {
                // RTerm = PExpr, "\'", ",", PExpr => ActionFn(124);
                let __sym3 = __pop_NtPExpr(__symbols);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_Term_22_5c_27_22(__symbols);
                let __sym0 = __pop_NtPExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action124::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtRTerm(__nt), __end));
                30
            }
            83 => {
                // SArith = SArith, "+", SFactor => ActionFn(22);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            84 => {
                // SArith = SArith, "-", SFactor => ActionFn(23);
                let __sym2 = __pop_NtSFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            85 => {
                // SArith = SFactor => ActionFn(24);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSArith(__nt), __end));
                31
            }
            86 => {
                // SExpr = SExpr, "<", SArith => ActionFn(16);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            87 => {
                // SExpr = SExpr, "<=", SArith => ActionFn(17);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3c_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            88 => {
                // SExpr = SExpr, ">", SArith => ActionFn(18);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3e_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            89 => {
                // SExpr = SExpr, ">=", SArith => ActionFn(19);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3e_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            90 => {
                // SExpr = SExpr, "==", SArith => ActionFn(20);
                let __sym2 = __pop_NtSArith(__symbols);
                let __sym1 = __pop_Term_22_3d_3d_22(__symbols);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            91 => {
                // SExpr = SArith => ActionFn(21);
                let __sym0 = __pop_NtSArith(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpr(__nt), __end));
                32
            }
            92 => {
                // SExpr? = SExpr => ActionFn(71);
                let __sym0 = __pop_NtSExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action71::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpr_3f(__nt), __end));
                33
            }
            93 => {
                // SExpr? =  => ActionFn(72);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action72::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpr_3f(__nt), __end));
                33
            }
            94 => {
                // SFactor = SFactor, "*", STerm => ActionFn(25);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            95 => {
                // SFactor = SFactor, "/", STerm => ActionFn(26);
                let __sym2 = __pop_NtSTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtSFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            96 => {
                // SFactor = STerm => ActionFn(27);
                let __sym0 = __pop_NtSTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSFactor(__nt), __end));
                34
            }
            97 => {
                // SIdent = r#"\\$\\w+"# => ActionFn(65);
                let __sym0 = __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSIdent(__nt), __end));
                35
            }
            98 => {
                // STerm = "-", STerm => ActionFn(28);
                let __sym1 = __pop_NtSTerm(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            99 => {
                // STerm = Number => ActionFn(29);
                let __sym0 = __pop_NtNumber(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            100 => {
                // STerm = "(", SExpr, ")" => ActionFn(30);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtSExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            101 => {
                // STerm = SIdent => ActionFn(31);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            102 => {
                // STerm = SIdent, "[", Comma<Expr>, "]" => ActionFn(32);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtComma_3cExpr_3e(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtSIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSTerm(__nt), __end));
                36
            }
            103 => {
                // Statement = Definition, ";" => ActionFn(2);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            104 => {
                // Statement = Command, ";" => ActionFn(3);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            105 => {
                // Statement =  => ActionFn(4);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action4::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                37
            }
            106 => {
                // String = r#"\"([^\"\\\\]|\\\\[\"\\\\])*\""# => ActionFn(70);
                let __sym0 = __pop_Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtString(__nt), __end));
                38
            }
            107 => {
                // __Ident = Ident => ActionFn(1);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ident(__nt), __end));
                39
            }
            108 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_NtStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 41 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5croutes_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5croutes_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_21_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_21_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_28_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_5b_5c_22_5c_5c_5c_5c_5d_29_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_40_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_40_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_28_5c_5c_2e_5b0_2d9_5d_2b_29_3f_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5c_24_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5c_24_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_7e_5c_5cw_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_7e_5c_5cw_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cRIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cRIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cRIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cRIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Command, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDefinition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Definition, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDefinition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Line, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumber<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumber(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Route, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<RIdent>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Route, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSArith<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSArith(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Scalar>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SIdent, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Scalar, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtString<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtString(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ident<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ident(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Statement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Statement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Statement::parse_Statement;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::{Statement, Command, Definition, Expr, Scalar, Point, Line, Route, SIdent, PIdent, LIdent, RIdent, Ident};
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:!)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:\")((?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:[\"-\"\\\\-\\\\]))*(?u:\")",
                "^(?u:@)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:[0-9])+((?u:\\.)(?u:[0-9])+)?",
                "^(?u:\\$)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:\\~)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:\\&)",
                "^(?u:\')",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\-)",
                "^(?u:\\-\\-)",
                "^(?u:\\->)",
                "^(?u:/)",
                "^(?u::)",
                "^(?u:;)",
                "^(?u:<)",
                "^(?u:<=)",
                "^(?u:=)",
                "^(?u:==)",
                "^(?u:>)",
                "^(?u:>=)",
                "^(?u:\\[)",
                "^(?u:\\\\routes)",
                "^(?u:\\])",
                "^(?u:\\{)",
                "^(?u:\\|)",
                "^(?u:\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:!)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:\")((?u:[\u{0}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:[\"-\"\\\\-\\\\]))*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:@)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+((?u:\\.)(?u:[0-9])+)?").unwrap(),
                __regex::Regex::new("^(?u:\\$)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:\\~)(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:\\&)").unwrap(),
                __regex::Regex::new("^(?u:\')").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:\\-\\-)").unwrap(),
                __regex::Regex::new("^(?u:\\->)").unwrap(),
                __regex::Regex::new("^(?u:/)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:<=)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:==)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:>=)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\\\routes)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\|)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 31 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> Ident
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Definition, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Definition(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Command, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Command(__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Statement
{
    Statement::None
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<RIdent>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, String, usize),
) -> Command
{
    Command::Routes(__0, __1)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Scalar, usize),
) -> Definition
{
    Definition::Scalar(__0, __1)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Point, usize),
) -> Definition
{
    Definition::Point(__0, __1)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Line, usize),
) -> Definition
{
    Definition::Line(__0, __1)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, RIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Route, usize),
) -> Definition
{
    Definition::Route(__0, __1)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Ident>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Scalar, usize),
) -> Definition
{
    Definition::ScalarMacro(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Ident>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Point, usize),
) -> Definition
{
    Definition::PointMacro(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Ident>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Line, usize),
) -> Definition
{
    Definition::LineMacro(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Scalar, usize),
) -> Expr
{
    Expr::Scalar(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
) -> Expr
{
    Expr::Point(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Line, usize),
) -> Expr
{
    Expr::Line(__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.lt(r)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.le(r)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    r.lt(l)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    r.le(l)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.eq(r)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Scalar, usize),
) -> Scalar
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.sub(r)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Scalar, usize),
) -> Scalar
{
    (__0)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.mul(r)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Scalar, usize),
) -> Scalar
{
    l.div(r)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Scalar, usize),
) -> Scalar
{
    (__0)
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Scalar, usize),
) -> Scalar
{
    __0.neg()
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> Scalar
{
    Scalar::Num(__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Scalar
{
    (__0)
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SIdent, usize),
) -> Scalar
{
    Scalar::Ident(__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Scalar
{
    Scalar::Macro(__0, __1)
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Point, usize),
) -> Point
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Point, usize),
) -> Point
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
) -> Point
{
    (__0)
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> Point
{
    p.mul(s)
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, Scalar, usize),
) -> Point
{
    p.mul(s)
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, Scalar, usize),
) -> Point
{
    p.div(s)
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Line, usize),
) -> Point
{
    Point::intersection(__0, __1)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
) -> Point
{
    (__0)
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Scalar, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Point
{
    Point::Pair(__0, __1)
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Point, usize),
) -> Point
{
    __0.neg()
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Point
{
    (__0)
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PIdent, usize),
) -> Point
{
    Point::Ident(__0)
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Point
{
    Point::Macro(__0, __1)
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Point, usize),
) -> Line
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Point, usize),
) -> Line
{
    l.add(r)
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> Line
{
    l.parallel(p)
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> Line
{
    l.perpendicular(p)
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, Scalar, usize),
) -> Line
{
    l.offset(s)
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Line, usize),
) -> Line
{
    (__0)
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Line, usize),
) -> Line
{
    (__0)
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Point, usize),
) -> Line
{
    Line::between(__0, __1)
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Point, usize),
) -> Line
{
    Line::vector(__0, __1)
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Line, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Line
{
    (__0)
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LIdent, usize),
) -> Line
{
    Line::Ident(__0)
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LIdent, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Line
{
    Line::Macro(__0, __1)
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    (_, r, _): (usize, Route, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, o, _): (usize, ::std::option::Option<Scalar>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> Route
{
    r.extend(o, p)
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Route, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Route, usize),
) -> Route
{
    l.concat(r)
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Route, usize),
) -> Route
{
    (__0)
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Point, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, ::std::option::Option<Scalar>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Point, usize),
) -> Route
{
    Route::start(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SIdent, usize),
) -> Ident
{
    Ident::Scalar(__0)
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PIdent, usize),
) -> Ident
{
    Ident::Point(__0)
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LIdent, usize),
) -> Ident
{
    Ident::Line(__0)
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> SIdent
{
    SIdent::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> PIdent
{
    PIdent::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> LIdent
{
    LIdent::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> RIdent
{
    RIdent::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0.get(1 .. __0.len()-1).unwrap())
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Scalar, usize),
) -> ::std::option::Option<Scalar>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Scalar>
{
    None
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, ::std::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        },
    }
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        },
    }
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<RIdent>, usize),
    (_, e, _): (usize, ::std::option::Option<RIdent>, usize),
) -> Vec<RIdent>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        },
    }
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, RIdent, usize),
) -> ::std::option::Option<RIdent>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<RIdent>
{
    None
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<RIdent>
{
    vec![]
}

#[allow(unused_variables)]
fn __action79<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<RIdent>, usize),
) -> ::std::vec::Vec<RIdent>
{
    v
}

#[allow(unused_variables)]
fn __action80<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, RIdent, usize),
    (_, _, _): (usize, &'input str, usize),
) -> RIdent
{
    (__0)
}

#[allow(unused_variables)]
fn __action81<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> ::std::option::Option<Ident>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action82<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Ident>
{
    None
}

#[allow(unused_variables)]
fn __action83<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Ident>
{
    vec![]
}

#[allow(unused_variables)]
fn __action84<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
) -> ::std::vec::Vec<Ident>
{
    v
}

#[allow(unused_variables)]
fn __action85<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Ident
{
    (__0)
}

#[allow(unused_variables)]
fn __action86<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::option::Option<Expr>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action87<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Expr>
{
    None
}

#[allow(unused_variables)]
fn __action88<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
fn __action89<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
fn __action90<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
fn __action91<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action92<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action93<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action94<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action95<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, RIdent, usize),
) -> ::std::vec::Vec<RIdent>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action96<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<RIdent>, usize),
    (_, e, _): (usize, RIdent, usize),
) -> ::std::vec::Vec<RIdent>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action97<
    'input,
>(
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action90(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action98<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action90(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action99<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action88(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action100<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
    __1: (usize, ::std::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action89(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
>(
    input: &'input str,
    __0: (usize, Ident, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action85(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action85(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action103<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action83(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action104<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action84(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action105<
    'input,
>(
    input: &'input str,
    __0: (usize, RIdent, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<RIdent>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action80(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<RIdent>, usize),
    __1: (usize, RIdent, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<RIdent>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action80(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<RIdent>, usize),
) -> Vec<RIdent>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action78(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<RIdent>, usize),
    __1: (usize, ::std::option::Option<RIdent>, usize),
) -> Vec<RIdent>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action79(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action109<
    'input,
>(
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action86(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action87(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action86(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action87(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
    'input,
>(
    input: &'input str,
    __0: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action81(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ident>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action82(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action81(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action82(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
    'input,
>(
    input: &'input str,
    __0: (usize, RIdent, usize),
) -> Vec<RIdent>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<RIdent>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action77(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<RIdent>, usize),
    __1: (usize, RIdent, usize),
) -> Vec<RIdent>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<RIdent>, usize),
) -> Vec<RIdent>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action77(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
    'input,
>(
    input: &'input str,
    __0: (usize, Route, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Scalar, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Point, usize),
) -> Route
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action71(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action122<
    'input,
>(
    input: &'input str,
    __0: (usize, Route, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Point, usize),
) -> Route
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action72(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action123<
    'input,
>(
    input: &'input str,
    __0: (usize, Point, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Scalar, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Point, usize),
) -> Route
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action71(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action124<
    'input,
>(
    input: &'input str,
    __0: (usize, Point, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Point, usize),
) -> Route
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action72(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
