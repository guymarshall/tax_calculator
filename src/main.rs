mod user_input;

/*
    This program calculates the income tax, national insurance tax, and total tax that will be paid on the user's income.
*/

/*
function calculate_tax($bands, $monthly_salary) {
    $running_pay = $monthly_salary;
    $running_tax = 0.0;

    foreach ($bands as $band) {
        if ($band[0] >= 0.0) {
            $band[2] = min($running_pay, $band[0]);
        } else {
            $band[2] = $running_pay;
        }
        $running_pay -= $band[2];
        $running_tax += $band[1] * $band[2];
    }

    return $running_tax;
}
*/

fn main() {
    let monthly_salary: i32 = user_input::get_user_input("Enter monthly salary (£):");

    let income_tax_bands: Vec<Vec<f64>> = vec![
        vec![12570.0 / 12.0,            0.0,    0.0],   // allowances
        vec![(50270.0 - 12570.0) / 12.0,  0.2,    0.0],   // basic
        vec![(150000.0 - 50270.0) / 12.0, 0.4,    0.0],   // higher
        vec![-1.0,                    0.45,   0.0]    // above
    ];

    let national_insurance_bands: Vec<Vec<f64>> = vec![
        vec![1048.0,            0.0,    0.0],   // primary threshold
        vec![4189.0 - 1048.0,   0.1325, 0.0],   // upper earnings limit
        vec![-1.0,              0.0325, 0.0]    // above
    ];
}
/*
$income_tax = calculate_tax($income_tax_bands, $monthly_salary);
$national_insurance_tax = calculate_tax($national_insurance_bands, $monthly_salary);
$total_tax = $income_tax + $national_insurance_tax;

printf("Income Tax: £%d\n", $income_tax);
printf("National Insurance Tax: £%d\n", $national_insurance_tax);
printf("Total: £%d\n", $total_tax);
*/