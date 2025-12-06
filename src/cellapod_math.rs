use itertools::izip;
use std::fs;

pub fn main() {
    let bank = fs::read_to_string("./input/6_cellapod_math.txt").unwrap();
    let columns = Column::news_p1(&bank);
    let mut sum = 0;
    for column in columns {
        sum += column.result()
    }
    println!("Day 6: Math homework = {}", sum);
    // 6299564383938

    let columns = Column::news_p2(&bank);
    let mut sum = 0;
    for column in columns {
        //println!("{:?}", column);
        sum += column.result()
    }
    println!("Day 6: second part = {}", sum);
    // 160937539732274499017365939
    // 11950004808442
}

#[derive(Debug)]
struct Column {
    numbers: Vec<u128>,
    operation: char,
}

impl Column {
    pub fn news_p1(bank: &str) -> Vec<Self> {
        let bank = bank.trim();
        let rows = bank.split('\n');
        let mut operations = Vec::new();
        let mut numbers: Vec<Vec<u128>> = Vec::new();
        for row in rows {
            let cells = row.split(' ');
            let mut column_id = 0;
            for cell in cells {
                let number_result = cell.parse::<u128>();
                let column = numbers.get_mut(column_id);
                if let Ok(number) = number_result
                    && let Some(internal) = column
                {
                    internal.push(number);
                } else if let Ok(number) = number_result {
                    numbers.push(vec![number]);
                } else if cell == "+" || cell == "*" {
                    operations.push(cell.chars().nth(0).unwrap());
                } else {
                    continue;
                }
                column_id += 1;
            }
        }
        let mut output = Vec::new();

        for (column, operation) in izip!(numbers, operations) {
            output.push(Self {
                numbers: column,
                operation,
            })
        }

        output
    }
    pub fn result(&self) -> u128 {
        if self.operation == '*' {
            self.numbers.iter().product()
        } else {
            self.numbers.iter().sum()
        }
    }

    pub fn news_p2(bank: &str) -> Vec<Self> {
        let bank = &bank[..bank.len()];
        let rows = bank.split('\n');
        //let mut operations = Vec::new();
        let mut digits: Vec<Vec<char>> = Vec::new();
        for row in rows {
            let cells = row.chars().rev();
            for (column_id, cell) in cells.enumerate() {
                let column_option = digits.get_mut(column_id);
                if let Some(column) = column_option {
                    column.push(cell);
                } else {
                    digits.push(vec![cell]);
                }
            }
        }
        let mut temp_numbers = Vec::new();
        let mut operation_option = None;
        let mut output = Vec::new();
        for digit in digits.iter_mut() {
            let temp_option = digit.pop();
            if let Some(temp) = temp_option
                && temp != ' '
            {
                operation_option = Some(temp);
            }
            let number: String = digit.iter().collect();
            let number_result = number.trim().parse::<u128>();
            if let Ok(number) = number_result {
                temp_numbers.push(number);
            } else if let Some(operation) = operation_option {
                output.push(Self {
                    numbers: temp_numbers,
                    operation,
                });
                temp_numbers = Vec::new();
                operation_option = None;
            }
        }
        if let Some(operation) = operation_option {
            output.push(Self {
                numbers: temp_numbers,
                operation,
            });
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let bank = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let columns = Column::news_p1(bank);
        let mut sum = 0;
        for column in columns {
            sum += column.result()
        }
        assert_eq!(sum, 4277556)
    }

    #[test]
    fn test_example_2() {
        let bank = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let columns = Column::news_p2(bank);
        println!("{:?}", columns);
        let mut sum = 0;
        for column in columns {
            sum += column.result()
        }
        assert_eq!(sum, 3263827)
    }

    #[test]
    fn test_error_2() {
        let bank = "4324 2   73 531 \n1875 44  57 122 \n179  747 25 127 \n357  297 34 3595\n+    *   *  +   ";
        let columns = Column::news_p2(bank);
        assert_eq!(columns.len(), 4)
    }
}
