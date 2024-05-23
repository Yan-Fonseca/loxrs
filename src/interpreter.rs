use crate::token_type::TokenType;
use crate::token::{LiteralPossibleValues, Token};
use crate::environment::Environment;
use crate::error_hadling::runtime_error;
use crate::expr::*;
use crate::stmt::Stmt;

pub enum Value {
    Boolean(bool),
    Literal(LiteralPossibleValues)
}

pub struct Error {
    token: Option<Token>,
    message: String
}

impl Error {
    fn new(token: Option<Token>, message: String) -> Self {
        Error {
            token,
            message
        }
    }
}

pub struct Interpreter{
    pub environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            match statement {
                Stmt::Expr(expr) => {
                    match expr {
                        Expr::Assign(value) => {
                            if let Some(assign) = value {
                                let result =*assign.get_expression();

                                let result = self.environment.assign(assign.get_value().get_lexeme(), Some(result));

                                match result {
                                    Ok(_) => {},
                                    Err(e) => runtime_error(assign.get_value(), e),
                                }
                            }
                        },
                        _ => {}
                    }
                },
                Stmt::Print(expr) => {
                    let result = self.get_expression_value(expr);
                    match result {
                        Ok(value) => self.handle_ok_result(value),
                        Err(e) => self.handle_error_result(e),
                    }
                },
                Stmt::Var(name, value) => {
                    self.environment.define(name.get_lexeme(), value);
                },
            }
        }
    }
    
    fn handle_ok_result(&self, value: Option<Value>) {
        if let Some(val) = value {
            match val {
                Value::Boolean(val) => self.print_boolean(val),
                Value::Literal(literal_value) => self.print_literal(literal_value),
            }
        } else {
            println!("Nil");
        }
    }
    
    fn print_boolean(&self, val: bool) {
        if val {
            println!("true");
        } else {
            println!("false");
        }
    }
    
    fn print_literal(&self, literal_value: LiteralPossibleValues) {
        match literal_value {
            LiteralPossibleValues::StringValue(string_value) => println!("{}", string_value),
            LiteralPossibleValues::DoubleValue(double_value) => println!("{}", double_value),
        }
    }
    
    fn handle_error_result(&self, e: Error) {
        match e.token {
            Some(token) => runtime_error(token, e.message),
            None => panic!("{}", e.message),
        }
    }    

    pub fn get_expression_value(&self, expression: Expr) -> Result<Option<Value>, Error> {
        match expression {
            Expr::Literal(value) => {
                if let Some(val) = value {
                    return self.get_literal_value(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Value does not exist".to_string());
                    return Err(error_value);
                }
            },
            Expr::Grouping(value) => {
                if let Some(val) = value {
                    return self.get_group(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Group does not exist".to_string());
                    return Err(error_value);
                }
            },
            Expr::Unary(value) => {
                if let Some(val) = value {
                    return self.get_unary(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Unary expression does not exist".to_string());
                    return Err(error_value);
                }
            },
            Expr::Binary(value) => {
                if let Some(val) = value {
                    return self.get_binary_expression_result_value(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Binary expression does not exist".to_string());
                    return Err(error_value);
                }
            }
            Expr::Variable(value) => {
                if let Some(val) = value {
                    return self.get_variable_value(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Variable expression error".to_string());
                    return Err(error_value);
                }
            },
            Expr::Assign(value) => {
                if let Some(val) = value {
                    return self.get_assign_value(val);
                }
                else {
                    let error_value = Error::new(None, "[ERROR] Assign expression error".to_string());
                    return Err(error_value);
                }
            }
        }
    }

    fn get_literal_value(&self, val: Literal) -> Result<Option<Value>, Error> {
        match val.get_value().get_token_type() {
            TokenType::Number => {
                match val.get_value().get_literal() {
                    Some(value) => {
                        return Ok(Some(Value::Literal(value)))
                    },
                    None => {
                        let error_value = Error::new(Some(val.get_value()), "[ERROR] The Token is a Number, but his value is nil.".to_string());
                        return Err(error_value);
                    }
                };
            },

            TokenType::String => {
                match val.get_value().get_literal() {
                    Some(value) => {
                        return Ok(Some(Value::Literal(value)))
                    },
                    None => {
                        let error_value = Error::new(Some(val.get_value()), "[ERROR] The Token is a String, but his value is nil.".to_string());
                        return Err(error_value);
                    }
                };
            },

            TokenType::True => {
                return Ok(Some(Value::Boolean(true)));
            },

            TokenType::False => {
                return Ok(Some(Value::Boolean(false)));
            },

            TokenType::Nil => {
                return Ok(None);
            }

            _ => return Err(Error::new(Some(val.get_value()), "[ERROR] The Token is not a literal.".to_string())),
        }
    }

    fn get_variable_value(&self, variable: Variable) -> Result<Option<Value>, Error> {
        let result = self.environment.get(variable.get_value());
        
        match result {
            Ok(value) => {
                self.get_expression_value(value)
            },
            Err(e) => Err(Error::new(Some(variable.get_value()), format!("{}",e))),
        }
    }

    fn get_assign_value(&self, assign: Assign) -> Result<Option<Value>, Error> {
        let result = self.environment.get(assign.get_value());
        
        match result {
            Ok(value) => {
                self.get_expression_value(value)
            },
            Err(e) => Err(Error::new(Some(assign.get_value()), format!("{}",e))),
        }
    }

    fn get_group(&self, group: Grouping) -> Result<Option<Value>, Error> {
        return self.get_expression_value(*group.get_expression().clone());
    }

    fn get_unary(&self, expression: Unary) -> Result<Option<Value>, Error> {
        let expression_result = self.get_expression_value(*expression.get_expression().clone())?;
        let operator = expression.get_operator().get_token_type();

        match operator {
            TokenType::Minus => {
                if let Some(Value::Literal(value)) = expression_result {
                    if let LiteralPossibleValues::DoubleValue(val) = value {
                        return Ok(Some(Value::Literal(LiteralPossibleValues::DoubleValue(-val))));
                    }
                    else {
                        let error_value = Error::new(Some(expression.get_operator()), "[ERROR] Cannot use the '-' operator in a string".to_string());
                        return Err(error_value);
                    }
                }
                else {
                    let error_value = Error::new(Some(expression.get_operator()), "[ERROR] Cannot use the '-' operator in a nil".to_string());
                    return Err(error_value);
                }
            },

            TokenType::Bang => {
                Ok(Some(Value::Boolean(!self.is_truthy(expression_result))))
            },

            _ => return Err(Error::new(Some(expression.get_operator()), "[ERROR] The token is unary, but do not have an unary operator!".to_string()))
        }
    }

    fn is_truthy(&self, value: Option<Value>) -> bool {
        match value {
            Some(val) => {
                match val {
                    Value::Boolean(value1) => value1,
                    Value::Literal(_) => true,
                }
            },
            None => false
        }
    }

    fn get_binary_expression_result_value(&self, value: Binary) -> Result<Option<Value>, Error> {
        let left = self.get_expression_value(*value.get_left().clone())?;
        let right = self.get_expression_value(*value.get_right().clone())?;
        let operator = value.get_operator().get_token_type();

        match operator {
            TokenType::Minus => {
                let result = self.subtract(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::Star => {
                let result = self.multiply(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::Slash => {
                let result = self.divide(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::Plus => {
                let result = self.sum(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },

            TokenType::Greater => {
                let result = self.greater(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::GreaterEqual => {
                let result = self.greater_equal(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::Less => {
                let result = self.less(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::LessEqual => {
                let result = self.less_equal(left, right);
                match result {
                    Ok(value) => Ok(value),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },

            TokenType::BangEqual => {
                let result = self.is_equal(left, right);
                match result {
                    Ok(value) => Ok(Some(Value::Boolean(!value))),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            TokenType::EqualEqual => {
                let result = self.is_equal(left, right);
                match result {
                    Ok(value) => Ok(Some(Value::Boolean(value))),
                    Err(message) => Err(Error::new(Some(value.get_operator()), message)),
                }
            },
            _ => Err(Error::new(Some(value.get_operator()), "[ERROR] Operator does not exist!".to_string()))
        }
    }

    fn subtract(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {

                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.subtract_aux(literal_left, literal_right);
                    }
                    _ => Err("[ERROR] Cannot subtract boolean values".to_string()),
                }

            }
            _ => Err("[ERROR] Cannot subtract nil values".to_string())
        }
    }

    fn subtract_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(number_left), LiteralPossibleValues::DoubleValue(number_right)) => {
                return Ok(Some(Value::Literal(LiteralPossibleValues::DoubleValue(number_left - number_right))));
            },
            _ => Err("[ERROR] Cannot subtract string values".to_string()),
        }
    }

    fn multiply(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.multiply_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot multiply boolean values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot multiply nil expressions.".to_string())
        }
    }

    fn multiply_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(number_left), LiteralPossibleValues::DoubleValue(number_right)) => {
                return Ok(Some(Value::Literal(LiteralPossibleValues::DoubleValue(number_left * number_right))));
            },
            _ => Err("[ERROR] Cannot multiply string values".to_string()),
        }
    }

    fn divide(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.divide_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot divide boolean values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot divide nil expressions.".to_string())
        }
    }

    fn divide_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(number_left), LiteralPossibleValues::DoubleValue(number_right)) => {
                return Ok(Some(Value::Literal(LiteralPossibleValues::DoubleValue(number_left / number_right))));
            },
            _ => Err("[ERROR] Cannot divide string values".to_string()),
        }
    }

    fn sum(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.sum_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot sum boolean values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot sum nil expressions.".to_string())
        }
    }

    fn sum_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::StringValue(value_left), LiteralPossibleValues::StringValue(value_right)) => {
                return Ok(Some(Value::Literal(LiteralPossibleValues::StringValue(value_left + value_right.as_str()))))
            },
            (LiteralPossibleValues::DoubleValue(value_left), LiteralPossibleValues::DoubleValue(value_right)) => {
                return Ok(Some(Value::Literal(LiteralPossibleValues::DoubleValue(value_left + value_right))));
            },

            _ => Err("[ERROR] Cannot sum Strings and numbers".to_string())
        }
    }

    fn greater(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.greater_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot compare boolean values with literal values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot compare nil expressions.".to_string())
        }
    }

    fn greater_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(value_left), LiteralPossibleValues::DoubleValue(value_right)) => {
                return Ok(Some(Value::Boolean(value_left > value_right)))
            },
            _ => Err("[ERROR] Cannot compare string values".to_string())
        }
    }

    fn greater_equal(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.greater_equal_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot compare boolean values with literal values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot compare nil expressions.".to_string())
        }
    }

    fn greater_equal_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(value_left), LiteralPossibleValues::DoubleValue(value_right)) => {
                return Ok(Some(Value::Boolean(value_left >= value_right)))
            },
            _ => Err("[ERROR] Cannot compare string values".to_string())
        }
    }

    fn less(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.less_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot compare boolean values with literal values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot compare nil expressions.".to_string())
        }
    }

    fn less_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(value_left), LiteralPossibleValues::DoubleValue(value_right)) => {
                return Ok(Some(Value::Boolean(value_left < value_right)))
            },
            _ => Err("[ERROR] Cannot compare string values".to_string())
        }
    }

    fn less_equal(&self, left: Option<Value>, right: Option<Value>) -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(value_left), Some(value_right)) => {
                match (value_left, value_right) {
                    (Value::Literal(literal_left), Value::Literal(literal_right)) => {
                        return self.less_equal_aux(literal_left, literal_right);
                    },
                    _ => Err("[ERROR] Cannot compare boolean values with literal values".to_string())
                }
            },
            _ => Err("[ERROR] Cannot compare nil expressions.".to_string())
        }
    }

    fn less_equal_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<Option<Value>, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(value_left), LiteralPossibleValues::DoubleValue(value_right)) => {
                return Ok(Some(Value::Boolean(value_left <= value_right)))
            },
            _ => Err("[ERROR] Cannot compare string values".to_string())
        }
    }

    // Bang Equal and Equal Equal Operations

    fn is_equal(&self, left: Option<Value>, right: Option<Value>) -> Result<bool, String> {
        match (left, right) {
            (None, None) => Ok(true),
            (None, Some(_)) => Ok(false),
            (Some(_), None) => Ok(false),
            (Some(left_value), Some(right_value)) => {
                match (left_value, right_value) {
                    (Value::Boolean(value_l), Value::Boolean(value_r)) => Ok(value_l == value_r),
                    (Value::Literal(value_l), Value::Literal(value_r)) => self.is_equal_aux(value_l, value_r),
                    _ => Err("[ERROR] Cannot compare booleans with literals".to_string())
                }
            },
        }
    }

    fn is_equal_aux(&self, left: LiteralPossibleValues, right: LiteralPossibleValues) -> Result<bool, String> {
        match (left, right) {
            (LiteralPossibleValues::DoubleValue(number_left), LiteralPossibleValues::DoubleValue(number_right)) => {
                Ok(number_left == number_right)
            },
            _ => Err("[ERROR] Cannot compare string values".to_string())
        }
    }

}