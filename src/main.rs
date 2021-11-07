use std::io::{ stdin, stdout, Write};

fn main() {
    println!("Quadratic equation solver by Qumolama.d | https://github.com/Lama3L9R/solve_quadratic");
    println!("I need the basic information of the equation: 0 = ax^2 + bx + c Where a, b, c is constant and a, b, c are Reals, also a is not equals to 0");
    let a = read_number("(Press Enter to confirm) a = ", false, 0.0);
    let b = read_number("(Press Enter to confirm) b = ", false, 0.0);
    let c = read_number("(Press Enter to confirm) c = ", false, 0.0);

    if a == 0.0 {
        panic!("I've told you that a is *not* equals to 0!!")
    }

    println!("Is this correct? {}x^2 + {}x + {} = 0", a, b, c);
    if !read_bool("[Y/yes for yes and others for no]: ") {
        print!("\033[2J");
        main();
    }

    let mut use_newton = true;
    let mut iter_count: i64 = 100;
    let mut init_value_guess1: f64 = 30.0;
    let mut init_value_guess2: f64 = -30.0;
    let mut print_function_info = true;

    let advanced_mode = read_bool("Do you want to use advanced mode? [y/yes for yes and others for no]: ");

    if advanced_mode {
        use_newton = read_bool("Do you want to use Newton's method? [y/yes for yes and others for no]: ");
        if use_newton {
            iter_count = read_int("How many iterations do you want to use? [Default: 100]: ", true, 100);
            init_value_guess1 = read_number("What's your initial guess of the root? [Default: 30]: ", true, 30.0);
            init_value_guess2 = read_number("(Only works when delta is not equals to 0)What's your another initial guess of the root? [Default: -30]: ", true, -30.0);
        }
        print_function_info = read_bool(
            "Do you want to print the function info? [Y/yes for yes and others for no]: ",
        );
    }

    let real_roots = solve_quadratic(a, b, c, use_newton, iter_count, init_value_guess1, init_value_guess2);
    
    println!("\n\nCalculation configs: ");
    if use_newton {
        println!("  Roots: Newton's method");
        println!("  Iterations: {}", iter_count);
        println!("  Initial guess 1: {}", init_value_guess1);
        println!("  Initial guess 2: {}", init_value_guess2);
    } else {
        println!("  Roots: Quadratic Roots formula with std sqrt()");
    }
    
    println!("\nThe information about equation {}x^2 + {}x + {} = 0: ", a, b, c);
    print!("  Roots: ");
    if real_roots.len() == 0 {
        println!("No real roots found.");
    } else if real_roots.len() == 1 {
        println!("The approximate value of the root is: {}", real_roots[0]);
    } else {
        println!("The approximate value of the roots are: {} and {}", real_roots[0], real_roots[1]);
    }
    println!("  Delta = {}^2 - 4 * {} * {} = {}", b, a, c, b.powi(2) - 4.0 * a * c);
    
    if print_function_info {
        println!("\nThe information about Function f(x) = {}x^2 + {}x + {}: ", a, b, c);
        let max = (4.0 * a * c - b.powi(2)) / 4.0 * a;
        if a > 0.0 {
            println!("  The Opening is: Upwards");
            println!("  The Maximum is: +Infinity");
            println!("  The Minimum is: {}", max);
        } else {
            println!("  The Opening is: Downwards");
            println!("  The Maximum is: {}", max);
            println!("  The Minimum is: -Infinity");
        }
        
        println!("  The symmetry axis is line x = {}", -b / (2.0 * a));
        println!("  The vertex is at ({}, {})", -b / (2.0 * a), max);
        
        if a > 0.0 {
            println!("  On the left of the symmetry axis y decreases as x increases");
            println!("  On the right of the symmetry axis y increases as x increases");
        } else {
            println!("  On the left of the symmetry axis y increases as x increases");
            println!("  On the right of the symmetry axis y decreases as x increases");
        }
        
        println!("  The derivatives of this function is f'(x) = 2{}x + {}", a, b);
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64, use_newton: bool, iter_count: i64, init_value1: f64, init_value2: f64) -> Vec<f64> {
    let mut roots: Vec<f64> = Vec::new();
    let delta = f64::powi(b, 2) - 4.0 * a * c;
    if delta < 0.0 {
        return roots;
    }
    
    if use_newton {
        roots.push(do_newton_iteration(a, b, c, iter_count, init_value1));
        if delta != 0.0 {
            roots.push(do_newton_iteration(a, b, c, iter_count, init_value2));
        }
    } else {
        roots.push((-b + delta.sqrt()) / (2.0 * a));
        if delta != 0.0 {
            roots.push((-b - delta.sqrt()) / (2.0 * a));
        }
    }
    
    return roots;
}

fn do_newton_iteration(a: f64, b: f64, c: f64, iter_count: i64, init_value: f64) -> f64 {
    if iter_count == 0 {
        return init_value;
    }
    
    return do_newton_iteration(a, b, c, iter_count - 1, (a * (f64::powi(init_value, 2)) - c) / (2.0 * a * init_value + b)); // What the fuck?
}

fn read_bool(prompt: &str) -> bool {
    print!("{}", prompt);
    stdout().flush().unwrap();
    
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read your input");
    input = input.trim().to_string().to_ascii_lowercase();
    if input == "y" || input == "yes" {
        return true;
    } else {
        return false;
    }
}

fn read_number(prompt: &str, usedefault: bool, default: f64) -> f64 {
    print!("{}", prompt);
    stdout().flush().unwrap();
    
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read your input");
        if usedefault {
            if input.contains("/") {
                let mut splited = input.split("/").into_iter().map(|it| { 
                    return it.trim().parse::<f64>().unwrap_or_else(|_| {
                        panic!("Failed to parse fractions!")
                    });
                }).collect::<Vec<f64>>();
                
                return splited.swap_remove(0) / splited.into_iter().product::<f64>();
            }
            
            return input
                .trim()
                .parse::<f64>()
                .unwrap_or_else(|_| {
                    println!("Input is not a float number! Using default: {}", default);
                    return default;
                });
        } else {
            if input.contains("/") {
                let mut splited = input.split("/").into_iter().map(|it| { 
                    return it.trim().parse::<f64>().unwrap_or_else(|_| {
                        panic!("Failed to parse fractions!")
                    });
                }).collect::<Vec<f64>>();
                
                return splited.swap_remove(0) / splited.into_iter().product::<f64>();
            }
        
            return input.trim().parse::<f64>()
            .expect("Input illegal! Please input a number that is in range 1.7977e+308 to 4.9407e-324");
        }
}

fn read_int(prompt: &str, usedefault: bool, default: i64) -> i64 {
    print!("{}", prompt);
    stdout().flush().unwrap();
    
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read your input");
    if usedefault {
        if input.contains("/") {
            let mut splited = input.split("/").into_iter().map(|it| { 
                return it.trim().parse::<i64>().unwrap_or_else(|_| {
                    panic!("Failed to parse fractions!")
                });
            }).collect::<Vec<i64>>();
            
            return splited.swap_remove(0) / splited.into_iter().product::<i64>();
        }
    
        return input
        .trim()
        .parse::<i64>()
        .unwrap_or_else(|_| {
            println!("Input illegal! Using default: {}", default);
            return default;
        });
    } else {
        if input.contains("/") {
            let mut splited = input.split("/").into_iter().map(|it| { 
                return it.trim().parse::<i64>().unwrap_or_else(|_| {
                    panic!("Failed to parse fractions!")
                });
            }).collect::<Vec<i64>>();
            
            return splited.swap_remove(0) / splited.into_iter().product::<i64>();
        }
    
        return input.trim().parse::<i64>()
        .expect("Input illegal! Please input a number that is in range -1e19 to 1e19");
    }
}
