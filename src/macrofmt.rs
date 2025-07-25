use std::result;

use pest::Parser;
use pest::iterators::Pairs;
#[derive(clap::Parser)]
struct Arg {
    input: String,
    output: String,
}
#[derive(pest_derive::Parser)]
#[grammar = "json.macro.pest"]
struct JsonMacroParser;
fn main() {
    use clap::Parser;
    let args = Arg::parse();
    let input = std::fs::read_to_string(&args.input).unwrap();
    let formated = format_json_macros(&input);
    std::fs::write(&args.output, formated).unwrap();
}

fn format_json_macros(input: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    let re = regex::Regex::new(r#"json!\s*\(([^)]*)\)"#).unwrap();
    for cap in re.captures_iter(input) {
        let full_match = cap.get(0).unwrap();
        let json_content = cap.get(1).unwrap();
        result.push_str(&input[last_end..full_match.start()]);
        let formated_json = format_json(json_content.as_str());
        result.push_str("json!(");
        result.push_str(&formated_json);
        result.push_str(")");
        last_end = full_match.end();
    }
    result.push_str(&input[last_end..]);
    //format_rust(&result)
    result
}

fn format_json(input: &str) -> String {
    dbg!(input.trim());
    let pairs = JsonMacroParser::parse(Rule::json, input.trim()).unwrap();
    format_pairs(pairs, 2)
}

fn format_pairs(pairs: Pairs<Rule>, indent: usize) -> String {
    let mut result = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::OBJ => {
                let pair_vec = pair.into_inner().collect::<Vec<_>>();
                if pair_vec.is_empty() {
                    result.push_str("{}");
                } else {
                    result.push_str("{\n");
                    for (i, kv_pair) in pair_vec.iter().enumerate() {
                        if kv_pair.as_rule() == Rule::KV {
                            let key_indent = " ".repeat(indent + 1);
                            let mut kv_inner = kv_pair.clone().into_inner();
                            if let Some(key) = kv_inner.next() {
                                if let Some(value) = kv_inner.next() {
                                    result.push_str(&key_indent);
                                    result.push_str(key.as_str());
                                    result.push_str(": ");
                                    result
                                        .push_str(&format_pairs(Pairs::single(value), indent + 1));
                                    if i < pair_vec.len() - 1 {
                                        result.push_str(",");
                                    }
                                    result.push_str("\n");
                                }
                            }
                        }
                    }
                    result.push_str(&" ".repeat(indent));
                    result.push_str("}");
                }
            }
            Rule::ARR => {
                let mut value = pair.into_inner().collect::<Vec<_>>();
                if value.is_empty() {
                    result.push_str("[]");
                } else {
                    result.push_str("[");
                    for (i, v) in value.iter().enumerate() {
                        let arr_indent = " ".repeat(indent + 1);
                        result.push_str(&arr_indent);
                        result.push_str(&format_pairs(Pairs::single(v.clone()), indent + 1));
                        if i < value.len() - 1 {
                            result.push(',');
                        }
                        result.push('\n');
                    }
                    result.push_str(&" ".repeat(indent));
                    result.push_str("]");
                }
            }
            Rule::RUST_VAR | Rule::RUST_NUM | Rule::STR_SLICE | Rule::boolean | Rule::null => {
                result.push_str(pair.as_str());
            }
            Rule::value => {
                result.push_str(&format_pairs(pair.into_inner(), indent));
            }
            _ => {
                // 对于其他规则，递归处理子规则
                result.push_str(&format_pairs(pair.into_inner(), indent));
            }
        }
    }
    result
}
