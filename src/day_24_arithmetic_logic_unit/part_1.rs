use std::collections::HashMap;

enum VariableOrNumber {
    Variable(usize),
    Number(i64),
}

impl VariableOrNumber {
    fn get_value(&self, variables: &[i64; 4]) -> i64 {
        match self {
            VariableOrNumber::Variable(index) => variables[*index],
            VariableOrNumber::Number(num) => *num,
        }
    }
}

enum Instruction {
    Inp(usize),
    Add(usize, VariableOrNumber),
    Mul(usize, VariableOrNumber),
    Div(usize, VariableOrNumber),
    Mod(usize, VariableOrNumber),
    Eql(usize, VariableOrNumber),
}

impl Instruction {
    fn execute(&self, variables: &mut [i64; 4], input: Option<i64>) {
        match self {
            Instruction::Inp(var) => variables[*var] = input.unwrap(),
            Instruction::Add(var, var_or_num) => variables[*var] += var_or_num.get_value(variables),
            Instruction::Mul(var, var_or_num) => variables[*var] *= var_or_num.get_value(variables),
            Instruction::Div(var, var_or_num) => variables[*var] /= var_or_num.get_value(variables),
            Instruction::Mod(var, var_or_num) => variables[*var] %= var_or_num.get_value(variables),
            Instruction::Eql(var, var_or_num) => {
                variables[*var] = (variables[*var] == var_or_num.get_value(variables)) as i64
            }
        }
    }
}

fn find_the_largest_fourteen_digit_model_number(
    instructions: &[Instruction],
    index: usize,
    variables: [i64; 4],
    memo: &mut HashMap<([i64; 4], usize), Option<String>>,
) -> Option<String> {
    if let Some(model_number) = memo.get(&(variables, index)) {
        return model_number.clone();
    }

    for model_digit in (1..=9).rev() {
        let mut next_variables = variables;
        let mut next_index = index;
        instructions[next_index].execute(&mut next_variables, Some(model_digit));
        next_index += 1;

        while let Some(instruction) = instructions.get(next_index) {
            if matches!(instructions[next_index], Instruction::Inp(_)) {
                if let Some(model_number) = find_the_largest_fourteen_digit_model_number(
                    instructions,
                    next_index,
                    next_variables,
                    memo,
                ) {
                    return Some(format!("{}{}", model_digit, model_number));
                } else {
                    break;
                }
            } else {
                instruction.execute(&mut next_variables, None);
                next_index += 1;
            }
        }

        if next_variables[3] == 0 {
            return Some(model_digit.to_string());
        }
    }

    memo.insert((variables, index), None);
    None
}

fn convert_to_instruction(instruction: &str) -> Instruction {
    let instruction = instruction.split_whitespace().collect::<Vec<_>>();

    let variables_index = |var: &str| var.chars().next().unwrap() as usize - b'w' as usize;
    let var_or_num = |operand: &str| {
        operand
            .parse::<i64>()
            .map(VariableOrNumber::Number)
            .unwrap_or_else(|_| VariableOrNumber::Variable(variables_index(operand)))
    };

    match instruction[..] {
        ["inp", var] => Instruction::Inp(variables_index(var)),
        ["mul", var, operand] => Instruction::Mul(variables_index(var), var_or_num(operand)),
        ["add", var, operand] => Instruction::Add(variables_index(var), var_or_num(operand)),
        ["mod", var, operand] => Instruction::Mod(variables_index(var), var_or_num(operand)),
        ["div", var, operand] => Instruction::Div(variables_index(var), var_or_num(operand)),
        ["eql", var, operand] => Instruction::Eql(variables_index(var), var_or_num(operand)),
        _ => unreachable!(),
    }
}

fn process_data(path: &str) -> Vec<Instruction> {
    let file_content = std::fs::read_to_string(path).unwrap();
    file_content
        .lines()
        .map(convert_to_instruction)
        .collect::<Vec<Instruction>>()
}

pub fn solve() -> String {
    let instructions = process_data("./input/24.txt");
    let mut memo = HashMap::new();
    let result = find_the_largest_fourteen_digit_model_number(&instructions, 0, [0; 4], &mut memo);
    format!(
        "Day 24: Arithmetic Logic Unit (Part 1) answer: {}",
        result.unwrap().parse::<usize>().unwrap()
    )
}
