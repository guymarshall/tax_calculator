mod user_input;

/*
    This program calculates the income tax, national insurance tax, and total tax that will be paid on the user's income.
*/

fn calculate_tax(bands: &mut Vec<Vec<f64>>, monthly_salary: f64) -> f64 {
    let mut running_pay: f64 = monthly_salary;
    let mut running_tax: f64 = 0.0;

    for band in bands {
        if band[0] >= 0.0 {
            band[2] = if running_pay < band[0] {
                running_pay
            } else {
                band[0]
            };
        } else {
            band[2] = running_pay;
        }
        running_pay -= band[2];
        running_tax += band[1] * band[2];
    }

    running_tax
}

fn main() {
    let monthly_salary: f64 = user_input::get_user_input("Enter monthly salary (£):");

    let mut income_tax_bands: Vec<Vec<f64>> = vec![
        vec![12570.0 / 12.0,            0.0,    0.0],   // allowances
        vec![(50270.0 - 12570.0) / 12.0,  0.2,    0.0],   // basic
        vec![(150000.0 - 50270.0) / 12.0, 0.4,    0.0],   // higher
        vec![-1.0,                    0.45,   0.0]    // above
    ];

    let mut national_insurance_bands: Vec<Vec<f64>> = vec![
        vec![1048.0,            0.0,    0.0],   // primary threshold
        vec![4189.0 - 1048.0,   0.1325, 0.0],   // upper earnings limit
        vec![-1.0,              0.0325, 0.0]    // above
    ];

    let income_tax: f64 = calculate_tax(&mut income_tax_bands, monthly_salary);
    let national_insurance_tax: f64 = calculate_tax(&mut national_insurance_bands, monthly_salary);
    let total_tax: f64 = income_tax + national_insurance_tax;

    println!("Income Tax: £{}", income_tax);
    println!("National Insurance Tax: £{}", national_insurance_tax);
    println!("Total: £{}", total_tax);
}