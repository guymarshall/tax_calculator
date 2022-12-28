/*
    This program calculates the income tax, national insurance tax, and total tax that will be paid on the user's income.
*/

fn main() {
    println!("Hello, world!");
}

/*
<?php

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

$monthly_salary = readline("Enter monthly salary: £");

if ($monthly_salary < 0) {
    exit("Please enter a valid monthly salary.");
}

$income_tax_bands = [
    [12570 / 12,            0.0,    0.0],   // allowances
    [(50270 - 12570) / 12,  0.2,    0.0],   // basic
    [(150000 - 50270) / 12, 0.4,    0.0],   // higher
    [-1,                    0.45,   0.0]    // above
];

$national_insurance_bands = [
    [1048,          0.00,   0.00],  // primary threshold
    [4189 - 1048,   0.1325, 0.00],  // upper earnings limit
    [-1.0,          0.0325, 0.00]   // above
];

$income_tax = calculate_tax($income_tax_bands, $monthly_salary);
$national_insurance_tax = calculate_tax($national_insurance_bands, $monthly_salary);
$total_tax = $income_tax + $national_insurance_tax;

printf("Income Tax: £%d\n", $income_tax);
printf("National Insurance Tax: £%d\n", $national_insurance_tax);
printf("Total: £%d\n", $total_tax);
*/