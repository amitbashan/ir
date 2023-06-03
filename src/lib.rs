use lalrpop_util::lalrpop_mod;

pub mod ir;

lalrpop_mod!(pub grammar);

#[test]
fn test() {
    let script = r#"
        b1 {
            const x0, 1
            const x1, 2
            const x2, 4
            add x3, x0, x1
            le x4, x3, x2
            tst x4
        }

        b2 {
            const x5, 0
        }

        b3 {
            const x6, 1
        }

        b4 {
            phi x7, x5, x6
            ret x7
        }

        b1 -> b2, b3
        b2 -> b4
        b3 -> b4
    "#;
    let result = grammar::ScriptParser::new().parse(script);

    println!("{:?}", result);
}