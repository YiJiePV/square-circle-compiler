//Author: Karena Qian
//Class: CSC 3310
//Professor: Carlos Arias
//Main functionality completed: 10/9/2022
//Extra credit completed: 10/10/2022
//Rust Assignment -> Lexical and Syntax Analyzer
/*Performs lexical and syntax analysis on input file based on following BNF grammar:
PROGRAM     -->   definitions: 
                     DEFS
                  operations:
                     OPERATIONS
                  end.
DEFS        -->   DEF | DEF; DEFS
DEF         -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)
OPERATIONS  -->   OPERATION | OPERATION; OPERATIONS
OPERATION   -->   print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)
ID          -->   LETTER+
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
NUM         -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9 */
//Also translates program into Scheme or Prolog and writes a .pt file containing a HTML representation of the program's parse tree
//NOTE: Points are NOT to be printed, contained, or intersects (Extra Semantic)
use std::env;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

//enums:
enum DefinitionType {
    POINT,
    CIRCLE,
    SQUARE,
    NONE        
}
enum OperationType{
    PRINT,
    CONTAINED,
    INTERSECTS,
    UNDEFINED
}

//structs:
pub struct Definition {
    def_type: DefinitionType,
    param1: String,
    param2: String
}
pub struct Operation {
    op_type: OperationType,
    param1: String,
    param2: String
}
//Main Program
fn main() {
    //Symbol Table (for Scheme and Prolog generation)
    let mut defs: HashMap<String, Definition> = HashMap::new(); //keys = ID, value = definition struct
    let mut ops: Vec<Operation> = Vec::new(); //operations vector
     
    //Get File Input
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let query = &args[2];
    let contents = fs::read_to_string(file_path).expect("Error: File Not Readable.");
    //outputs based on tag
    if query.eq("-s"){
        println!("; processing input file {}", file_path);
    }
    else if query.eq("-p"){
        println!("/* processing input file {}", file_path);
    }
    /* //Pass file input into Syntax analyzer*/
    let tree_rep = program(&contents, &mut defs, &mut ops);
    //outputs based on tag
    if query.eq("-s"){
        println!("; Lexical and Syntax analysis passed");
        output_scheme(&defs, &ops);
    }
    else if query.eq("-p"){
        println!("\tLexical and Syntax analysis passed */");
        output_prolog(&defs, &ops);
    }
    //create new file for HTML tree
    let mut new_file = file_path.split_at(file_path.len() - 3).0.to_string();
    new_file.push_str(".pt");
    let mut tree_file = File::create(new_file).expect("Error encountered while creating file!");
    //write tree to file
    tree_file.write_all(tree_rep.as_bytes()).expect("Error while writing to file");
}

//Main Program Syntax Analyzer >:)
//Checks the following grammar:
/*PROGRAM     -->   definitions: 
                        DEFS
                    operations:
                        OPERATIONS
                    end.*/
/*Parameters:
    source: &str -> source code of program
    defs: &mut HashMap<String, Definition> -> storage for variable definitions
    op_v: &mut Vec<Operation> -> storage for operations*/
//Returns: HTML tree representation of source code program
fn program(source: &str, defs: &mut HashMap<String, Definition>, op_v: &mut Vec<Operation>)->String{
    //<program>
    let mut html = String::from("<program>\n"); //HTML tree representation of program (mainly for debugging)
    
    //Gets next token
    let mut tup = lex(source);
    let mut lex_tok = tup.1;
    let mut code = tup.0;
    //If lex is not definitions -> PANIC!
    if !lex_tok.eq("DEFINITIONS"){
        panic!("Syntax Error: definitions or colon expected");
    }
    //  -definition
    html.push_str("\t-definitions\n");

    //Gets next token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
    //If lex is not colon -> PANIC!
    if !lex_tok.eq("COLON"){
        panic!("Syntax Error: colon expected after definitions");
    }
    //  -colon
    //  <defs>
    html.push_str("\t-colon\n\t<defs>\n");

    //Check for empty definitions section
    tup = lex(code.as_str());
    lex_tok = tup.1;
    if lex_tok.eq("OPERATIONS"){
        code = tup.0;
    }
    else{
        //iterate through definitions
        loop{
            //pass source-code and defs HashMap to def function
            tup = def(&code, defs);
            code = tup.0;
            html.push_str(tup.1.as_str()); //add subtree to html
            //get next token
            tup = lex(&code);
            lex_tok = tup.1;
            code = tup.0;
            //if lex is not semicolon -> BREAK
            if !lex_tok.eq("SEMICOLON"){
                break;
            }
            //      -semicolon
            html.push_str("\t\t-semicolon\n");
        }
        //  </defs>
        html.push_str("\t</defs>\n");

        //If lex is not operations -> PANIC!
        if !lex_tok.eq("OPERATIONS"){
            panic!("Syntax Error: operations, colon, or semicolon expected");
        }
    }
    //  -operations
    html.push_str("\t-operations\n");

    //Get next token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
    //If lex is not colon -> PANIC!
    if !lex_tok.eq("COLON"){
        panic!("Error: colon expected after operations");
    }
    //  -colon
    //  <ops>
    html.push_str("\t-colon\n\t<ops>\n");

    //checks for empty operations section
    tup = lex(code.as_str());
    lex_tok = tup.1;
    if lex_tok.eq("END"){
        code = tup.0;
    }
    else{
        //iterate through operations
        loop{
            //pass source-code and operations vector to ops function
            let op = ops(code.as_str(), op_v);
            code = op.0.to_string();
            html.push_str(op.1.as_str()); //add subtree to html
            //Get next token
            tup = lex(code.as_str());
            lex_tok = tup.1;
            code = tup.0;
            //If lex is not semicolon -> BREAK
            if !lex_tok.eq("SEMICOLON"){
                break;
            }
            //      -semicolon
            html.push_str("\t\t-semicolon\n");
        }
        //  </ops>
        html.push_str("\t</ops>\n");

        //If lex is not end -> PANIC!
        if !lex_tok.eq("END"){
            panic!("Syntax Error: end, period, or semicolon expected");
        }
    }
    //  -end
    html.push_str("\t-end\n");

    //Get next lex/token
    tup = lex(code.as_str());
    lex_tok = tup.1;
	//If lex is not period -> PANIC!
    if !lex_tok.eq("PERIOD"){
        panic!("Syntax Error: period expected after end");
    }
    //  -period
    //</program>
    html.push_str("\t-period\n</program>");

    return html;
}

//Definition Syntax Analyzer
//Checks the following grammar:
/*DEF       -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)*/
/*Parameters:
    source: &str -> source code of program
    defs: &mut HashMap<String, Definition> -> storage for variable definitions */
//Returns: (source code remaining, HTML subtree representation)
fn def(source: &str, defs: &mut HashMap<String, Definition>)->(String, String){
    //      <def>
    let mut html = String::from("\t\t<def>\n");

    //Get token
    let mut tup = lex(source);
    let mut code = tup.0;
    let lex_tok = tup.1;
	//If lex is not ID -> PANIC!
    if !lex_tok.contains("ID"){
        panic!("Syntax Error: ID expected");
    }
    //get ID value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops ID token
    let id = iter.next().unwrap_or_default();
    //          <id>
    //              -idname
    //          </id>
    html.push_str("\t\t\t<id>\n\t\t\t\t-");
    html.push_str(id);
    html.push_str("\n\t\t\t</id>\n");

    //Get token
    tup = lex(code.as_str());
    let a_tok = tup.1;
    code = tup.0;
	//If lex is not Equal-sign -> PANIC!
    if !a_tok.eq("ASSIGN"){
        panic!("Syntax Error: assign sign expected after ID");
    }
    //          -assign
    html.push_str("\t\t\t-assign\n");

    //Get token
    tup = lex(code.as_str());
    let p_tok = tup.1;
    code = tup.0;
	//if lex is point -> 
    if p_tok.eq("POINT"){
        //pass source-code into point function
        let point = point(code.as_str());
        code = point.0.to_string();
        html.push_str(point.1.as_str()); //add subtree to html
        defs.insert(id.to_string(), point.2); //add definition to defs
    }
    else if p_tok.eq("CIRCLE") || p_tok.eq("SQUARE"){
        //pass token, source-code into circle_square fn
        let cir_squ = circle_square(&p_tok, code.as_str());
        code = cir_squ.0;
        html.push_str(cir_squ.1.as_str()); //add subtree to html
        defs.insert(id.to_string(), cir_squ.2); //add definition to defs
    }
    else{
        panic!("Syntax Error: point, circle, or square expected");
    }
    //      </def>
    html.push_str("\t\t</def>\n");

    return (code.to_string(), html); //(code left, HTML subtree)
}
//Circle-Square Syntax Analyzer
//Checks the following grammar:
//circle(ID, NUM) | square(ID, NUM)
/*Parameters:
    tok: &String -> token of circle or square
    source: &str -> source code of program */
//Returns: (source code remaining, HTML subtree representation, variable definition)
fn circle_square(tok: &String, source: &str)->(String, String, Definition){
    let mut html = String::new();

    let mut d = Definition{
        def_type: DefinitionType::NONE,
        param1: String::new(),
        param2: String::new()
    };

    if tok.eq("CIRCLE"){
        //          -circle
        html.push_str("\t\t\t-circle\n");
        d.def_type = DefinitionType::CIRCLE;
    }
    else if tok.eq("SQUARE"){
        //          -square
        html.push_str("\t\t\t-square\n");
        d.def_type = DefinitionType::SQUARE;
    }

    //Get token
    let mut tup = lex(source);
    let mut lex_tok = tup.1;
    let mut code = tup.0;
	//If lex is not lparen -> PANIC!
    if !lex_tok.eq("LPAREN"){
        panic!("Syntax Error: left parentheses expected");
    }
    //          -lparen
    html.push_str("\t\t\t-lparen\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not ID -> PANIC!
    if !lex_tok.contains("ID"){
        panic!("Syntax Error: ID expected");
    }
    //get ID value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops ID token
    let id = iter.next().unwrap_or_default();
    //          <id>
    //              -idname
    //          </id>
    html.push_str("\t\t\t<id>\n\t\t\t\t-");
    html.push_str(id);
    html.push_str("\n\t\t\t</id>\n");
    d.param1 = id.to_string();

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not comma -> PANIC!
    if !lex_tok.eq("COMMA"){
        panic!("Syntax Error: comma expected");
    }
    //          -comma
    html.push_str("\t\t\t-comma\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not NUM -> PANIC!
    if !lex_tok.contains("NUM"){
        panic!("Syntax Error: NUM expected");
    }
    //get NUM value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops NUM token
    let num = iter.next().unwrap_or_default();
    //          <num>
    //              -num
    //          </num>
    html.push_str("\t\t\t<num>\n\t\t\t\t-");
    html.push_str(num);
    html.push_str("\n\t\t\t</num>\n");
    d.param2 = num.to_string();

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not rparen -> PANIC!
    if !lex_tok.eq("RPAREN"){
        panic!("Syntax Error: right parentheses expected");
    }
    //          -rparen
    html.push_str("\t\t\t-rparen\n");

    return (code.to_string(), html, d); //(code left, HTML subtree, variable def)
}
//Point Syntax Analyzer
//Checks the following grammar:
//point(NUM, NUM)
/*Parameters:
    source: &str -> source code of program */
//Returns: (source code remaining, HTML subtree representation, variable definition)
fn point(source: &str)->(String, String, Definition){
    //          -point
    let mut html = String::from("\t\t\t-point\n");

    let mut d = Definition{
        def_type: DefinitionType::POINT,
        param1: String::new(),
        param2: String::new()
    };

    //Get token
    let mut tup = lex(source);
    let mut lex_tok = tup.1;
    let mut code = tup.0;
	//If lex is not lparen -> PANIC!
    if !lex_tok.eq("LPAREN"){
        panic!("Syntax Error: left parentheses expected");
    }
    //          -lparen
    html.push_str("\t\t\t-lparen\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not NUM -> PANIC!
    if !lex_tok.contains("NUM"){
        panic!("Syntax Error: NUM expected");
    }
    //get NUM value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops NUM token
    let num = iter.next().unwrap_or_default();
    //          <num>
    //              -num
    //          </num>
    html.push_str("\t\t\t<num>\n\t\t\t\t-");
    html.push_str(num);
    html.push_str("\n\t\t\t</num>\n");
    d.param1 = num.to_string();

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not comma -> PANIC!
    if !lex_tok.eq("COMMA"){
        panic!("Syntax Error: comma expected");
    }
    //          -comma
    html.push_str("\t\t\t-comma\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not NUM -> PANIC!
    if !lex_tok.contains("NUM"){
        panic!("Syntax Error: NUM expected");
    }
    //get NUM value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops NUM token
    let num = iter.next().unwrap_or_default();
    //          <num>
    //              -num
    //          </num>
    html.push_str("\t\t\t<num>\n\t\t\t\t-");
    html.push_str(num);
    html.push_str("\n\t\t\t</num>\n");
    d.param2 = num.to_string();

    //Get lex
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not rparen -> PANIC!
    if !lex_tok.eq("RPAREN"){
        panic!("Syntax Error: right parentheses expected");
    }
    //          -rparen
    html.push_str("\t\t\t-rparen\n");

    return (code, html, d); //(code left, HTML subtree, variable definition)
}

//Operations Syntax Analyzer
//Checks the following grammar:
/*OPERATION  -->  print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)*/
/*Parameters:
    source: &str -> source code of program
    ops: &mut Vec<Operation> -> storage for operations */
//Returns: (source code left, HTML subtree representation)
fn ops(source: &str, ops: &mut Vec<Operation>)->(String, String){
    //      <op>
    let mut html = String::from("\t\t<op>\n");

    //Get token
    let tup = lex(source);
    let lex_tok = tup.1;
    let mut code = tup.0;
    if lex_tok.eq("PRINT"){
        //pass source-code and ops into print_function
        let print = print(code.as_str(), ops);
        code = print.0.to_string();
        html.push_str(print.1.as_str()); //add subtree to html
    }
    else if lex_tok.eq("CONTAINED") || lex_tok.eq("INTERSECTS"){
        //pass lex, source-code, and ops into contain_inter function
        let con_int = contain_inter(&lex_tok, code.as_str(), ops);
        code = con_int.0;
        html.push_str(con_int.1.as_str()); //add subtree to html
    }
    else{
        panic!("Syntax Error: print, contained, or intersects expected");
    }

    //      </op>
    html.push_str("\t\t</op>\n");

    return (code, html); //(code left, HTML subtree)
}
//Contained-Intersects Syntax Analyzer
//Checks the following grammar:
//contained(ID, ID) | intersects(ID, ID)
/*Parameters:
    tok: &String -> token of contained or intersects
    source: &str -> source code of program
    ops: &mut Vec<Operation> -> storage for operations */
//Returns: (source code left, HTML subtree representation)
fn contain_inter(tok: &String, source: &str, ops: &mut Vec<Operation>)->(String, String){
    let mut html = String::new();

    let mut o = Operation{
        op_type: OperationType::UNDEFINED,
        param1: String::new(),
        param2: String::new()
    };

    if tok.eq("CONTAINED"){
        o.op_type = OperationType::CONTAINED;
        //          -contained
        html.push_str("\t\t\t-contained\n");
    }
    else if tok.eq("INTERSECTS"){
        o.op_type = OperationType::INTERSECTS;
        //          -intersects
        html.push_str("\t\t\t-intersects\n");
    }

    //Get token
    let mut tup = lex(source);
    let mut lex_tok = tup.1;
    let mut code = tup.0;
    //If lex is not lparen -> PANIC!
    if !lex_tok.eq("LPAREN"){
        panic!("Syntax Error: left parentheses expected");
    }
    //          -lparen
    html.push_str("\t\t\t-lparen\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not ID -> PANIC!
    if !lex_tok.contains("ID"){
        panic!("Syntax Error: ID expected");
    }
    //get ID value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops ID token
    let id = iter.next().unwrap_or_default();
    o.param1 = id.to_string();
    //          <id>
    //              -idname
    //          </id>
    html.push_str("\t\t\t<id>\n\t\t\t\t-");
    html.push_str(id);
    html.push_str("\n\t\t\t</id>\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not comma -> PANIC!
    if !lex_tok.eq("COMMA"){
        panic!("Syntax Error: comma expected");
    }
    //          -comma
    html.push_str("\t\t\t-comma\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not ID -> PANIC!
    if !lex_tok.contains("ID"){
        panic!("Syntax Error: ID expected");
    }
    //get ID value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops ID token
    let id = iter.next().unwrap_or_default();
    o.param2 = id.to_string();
    //          <id>
    //              -idname
    //          </id>
    html.push_str("\t\t\t<id>\n\t\t\t\t-");
    html.push_str(id);
    html.push_str("\n\t\t\t</id>\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not rparen -> PANIC!
    if !lex_tok.eq("RPAREN"){
        panic!("Syntax Error: right parentheses expected");
    }
    //          -rparen
    html.push_str("\t\t\t-rparen\n");
    
    ops.push(o); //add operations to vector

    return (code.to_string(), html); //(code left, HTML subtree)
}
//Print Syntax Analyzer
//Checks the following grammar:
//print(ID)
/*Parameters:
    source: &str -> source code of program
    ops: &mut Vec<Operation> -> storage for operations */
//Returns: (source code left, HTML subtree representation)
fn print(source: &str, ops: &mut Vec<Operation>)->(String, String){
    //          -print
    let mut html = String::from("\t\t\t-print\n");

    let mut o = Operation{
        op_type: OperationType::PRINT,
        param1: String::new(),
        param2: String::new()
    };

    //Get token
    let mut tup = lex(source);
    let mut lex_tok = tup.1;
    let mut code = tup.0;
	//If lex is not lparen -> PANIC!
    if !lex_tok.eq("LPAREN"){
        panic!("Syntax Error: left parentheses expected");
    }
    //          -lparen
    html.push_str("\t\t\t-lparen\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not ID -> PANIC!
    if !lex_tok.contains("ID"){
        panic!("Syntax Error: ID expected");
    }
    //get ID value
    let mut iter = lex_tok.split_ascii_whitespace();
    iter.next(); //drops ID token
    let id = iter.next().unwrap_or_default();
    o.param1 = id.to_string();
    //          <id>
    //              -idname
    //          </id>
    html.push_str("\t\t\t<id>\n\t\t\t\t-");
    html.push_str(id);
    html.push_str("\n\t\t\t</id>\n");

    //Get token
    tup = lex(code.as_str());
    lex_tok = tup.1;
    code = tup.0;
	//If lex is not rparen -> PANIC!
    if !lex_tok.eq("RPAREN"){
        panic!("Syntax Error: right parentheses expected");
    }
    //          -rparen
    html.push_str("\t\t\t-rparen\n");

    ops.push(o);

    return (code, html); //(code left, HTML subtree)
}

//Lexical Analyzer
//Gets one lexeme from source code and checks for lexical errors
/*Parameters:
    source: &str -> source code of program */
//Returns: (source code left, one token)
fn lex(source: &str)->(String, String){
    let mut lexeme = String::new(); //lex variable
    let mut is_id = false; //if lexeme is alpha
    let mut is_num = false; //if lexeme is digit
    let mut is_sym = false; //if lexeme is symbol
    let mut index = 0;
    for ch in source.chars(){
        //skip all white spaces
        if ch.is_ascii_whitespace(){
            index += 1;
            continue;
        }
        //error if char is uppercase
        if ch.is_ascii_uppercase(){
            panic!("Lexical Error: {}", ch);
        }
        //if digit lexeme
        if is_num && !ch.is_ascii_digit(){
            return (source.split_at(index).1.to_string(),String::from("NUM ") + lexeme.as_str()); //(code left, token)
        }
        //if alpha lexeme
        if is_id && !ch.is_ascii_lowercase(){
            return (source.split_at(index).1.to_string(),token(&lexeme, is_sym)); //(code left, token)
        }
        lexeme.push(ch); //build lexeme
        //check if char is alpha
        if ch.is_ascii_lowercase(){
            is_id = true;
        }   
        //check if char is digit
        else if ch.is_ascii_digit(){
            is_num = true;
        }
        //if non-digit non-alpha lexeme
        else{
            is_sym = true;
            return (source.split_at(index + 1).1.to_string(),token(&lexeme, is_sym)); //(code left, token)
        }
        index += 1;
    }
    return (String::from("No Code Left"), String::from("None"));
}
//Tokenizer
//Given lexeme returns token (excluding NUMs)
/*Parameters:
    lexeme: &String -> one lexeme
    is_sym: bool -> if lexeme is a symbol */
//Returns: corresponding token of lexeme
fn token(lexeme: &String, is_sym: bool) -> String{
    if lexeme == "definitions"{
        return String::from("DEFINITIONS");
    }
    else if lexeme == "operations"{
        return String::from("OPERATIONS");
    }
    else if lexeme == "point"{
        return String::from("POINT");
    }
    else if lexeme == "circle"{
        return String::from("CIRCLE");
    }
    else if lexeme == "square"{
        return String::from("SQUARE");
    }
    else if lexeme == "print"{
        return String::from("PRINT");
    }
    else if lexeme == "contained"{
        return String::from("CONTAINED");
    }
    else if lexeme == "intersects"{
        return String::from("INTERSECTS");
    }
    else if lexeme == "end"{
        return String::from("END");
    }
    else if lexeme == ";"{
        return String::from("SEMICOLON");
    }
    else if lexeme == ":"{
        return String::from("COLON");
    }
    else if lexeme == ","{
        return String::from("COMMA");
    }
    else if lexeme == "."{
        return String::from("PERIOD");
    }
    else if lexeme == "("{
        return String::from("LPAREN");
    }
    else if lexeme == ")"{
        return String::from("RPAREN");
    }
    else if lexeme == "="{
        return String::from("ASSIGN");
    }
    else if is_sym == false{
        return ("ID ").to_string() + lexeme.as_str();
    }
    else{
        panic!("Lexical Error: {}", lexeme);
    }
}

//Scheme Output
//Outputs given list of operations in Scheme
/*Parameters:
    defs: &HashMap<String, Definition> -> storage for variable definitions
    ops: &Vec<Operation> -> storage for operations */
//Returns: nothing
fn output_scheme(defs: &HashMap<String, Definition>, ops: &Vec<Operation>){
    for o in ops{
        match o.op_type{
            OperationType::CONTAINED =>{
                print!("(contained"); //Scheme
                let mut id = &o.param1;
                let p1 = defs.get(id.as_str()).unwrap(); //get first parameter of operation
                match p1.def_type{
                    DefinitionType::CIRCLE => {
                        print!("-circle"); //Scheme
                    },
                    DefinitionType::SQUARE => {
                        print!("-square"); //Scheme
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                id = &o.param2;
                let p2 = defs.get(id.as_str()).unwrap(); //get second parameter of operation
                match p2.def_type{
                    DefinitionType::CIRCLE => {
                        print!("-circle "); //Scheme
                    },
                    DefinitionType::SQUARE => {
                        print!("-square "); //Scheme
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p1_pt = defs.get(p1.param1.as_str()).unwrap(); //get point of first parameter
                print!("(makepoint {} {}) {} ", p1_pt.param1, p1_pt.param2, p1.param2); //Scheme
                let p2_pt = defs.get(p2.param1.as_str()).unwrap(); //get point of second parameter
                println!("(makepoint {} {}) {})", p2_pt.param1, p2_pt.param2, p2.param2); //Scheme

            }
            OperationType::INTERSECTS =>{
                print!("(intersects"); //Scheme
                let mut id = &o.param1;
                let p1 = defs.get(id.as_str()).unwrap(); //get first parameter of operation
                match p1.def_type{
                    DefinitionType::CIRCLE => {
                        print!("-circle"); //Scheme
                    },
                    DefinitionType::SQUARE => {
                        print!("-square"); //Scheme
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                id = &o.param2;
                let p2 = defs.get(id.as_str()).unwrap(); //get second parameter of operation
                match p2.def_type{
                    DefinitionType::CIRCLE => {
                        print!("-circle "); //Scheme
                    },
                    DefinitionType::SQUARE => {
                        print!("-square "); //Scheme
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p1_pt = defs.get(p1.param1.as_str()).unwrap(); //get point of first parameter
                print!("(makepoint {} {}) {} ", p1_pt.param1, p1_pt.param2, p1.param2); //Scheme
                let p2_pt = defs.get(p2.param1.as_str()).unwrap(); //get point of second parameter
                println!("(makepoint {} {}) {})", p2_pt.param1, p2_pt.param2, p2.param2); //Scheme
            }
            OperationType::PRINT =>{
                print!("(print"); //Scheme
                let id = &o.param1;
                let p = defs.get(id.as_str()).unwrap(); //get parameter of operation
                match p.def_type{
                    DefinitionType::CIRCLE => {
                        print!("-circle "); //Scheme
                    },
                    DefinitionType::SQUARE => {
                        print!("-square "); //Scheme
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p_pt = defs.get(p.param1.as_str()).unwrap(); //get point of parameter
                println!("(makepoint {} {}) {})", p_pt.param1, p_pt.param2, p.param2); //Scheme
            }
            OperationType::UNDEFINED =>{
                panic!("Error: undefined operation");
            }
        }
    }
}
//Prolog Output
//Outputs given list of operations in Prolog
/*Parameters:
    defs: &HashMap<String, Definition> -> storage for variable definitions
    ops: &Vec<Operation> -> storage for operations */
//Returns: nothing
fn output_prolog(defs: &HashMap<String, Definition>, ops: &Vec<Operation>){
    for o in ops{
        match o.op_type{
            OperationType::CONTAINED =>{
                print!("query(contained("); //Prolog
                let mut id = &o.param1;
                let p1 = defs.get(id.as_str()).unwrap(); //get first parameter of operation
                match p1.def_type{
                    DefinitionType::CIRCLE => {
                        print!("circle("); //Prolog
                    },
                    DefinitionType::SQUARE => {
                        print!("square("); //Prolog
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p1_pt = defs.get(p1.param1.as_str()).unwrap(); //get point of first parameter
                print!("point2d({},{}), {}), ", p1_pt.param1, p1_pt.param2, p1.param2); //Prolog
                id = &o.param2;
                let p2 = defs.get(id.as_str()).unwrap(); //get second parameter of operation
                match p2.def_type{
                    DefinitionType::CIRCLE => {
                        print!("circle("); //Prolog
                    },
                    DefinitionType::SQUARE => {
                        print!("square("); //Prolog
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p2_pt = defs.get(p2.param1.as_str()).unwrap(); //get point of second parameter
                println!("point2d({},{}), {}))).", p2_pt.param1, p2_pt.param2, p2.param2); //Prolog
            }
            OperationType::INTERSECTS =>{
                print!("query(intersects("); //Prolog
                let mut id = &o.param1;
                let p1 = defs.get(id.as_str()).unwrap(); //get first parameter of operation
                match p1.def_type{
                    DefinitionType::CIRCLE => {
                        print!("circle("); //Prolog
                    },
                    DefinitionType::SQUARE => {
                        print!("square("); //Prolog
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p1_pt = defs.get(p1.param1.as_str()).unwrap(); //get point of first parameter
                print!("point2d({},{}), {}), ", p1_pt.param1, p1_pt.param2, p1.param2); //Prolog
                id = &o.param2;
                let p2 = defs.get(id.as_str()).unwrap(); //get second parameter of operation
                match p2.def_type{
                    DefinitionType::CIRCLE => {
                        print!("circle("); //Prolog
                    },
                    DefinitionType::SQUARE => {
                        print!("square("); //Prolog
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p2_pt = defs.get(p2.param1.as_str()).unwrap(); //get point of second parameter
                println!("point2d({},{}), {}))).", p2_pt.param1, p2_pt.param2, p2.param2); //Prolog
            }
            OperationType::PRINT =>{
                print!("query("); //Prolog
                let id = &o.param1;
                let p = defs.get(id.as_str()).unwrap(); //get parameter of operation
                match p.def_type{
                    DefinitionType::CIRCLE => {
                        print!("circle("); //Prolog
                    },
                    DefinitionType::SQUARE => {
                        print!("square("); //Prolog
                    },
                    DefinitionType::POINT => {
                        panic!("Error: point cannot not be passed as param");
                    },
                    DefinitionType::NONE => {
                        panic!("Error: type not specified");
                    }
                }
                let p_pt = defs.get(p.param1.as_str()).unwrap(); //get point of parameter
                println!("point2d({},{}), {})).", p_pt.param1, p_pt.param2, p.param2); //Prolog
            }
            OperationType::UNDEFINED =>{
                panic!("Error: undefined operation");
            }
        }
    }
    println!("writeln(T) :- write(T), nl.\nmain:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),\n\thalt."); //Prolog  
}