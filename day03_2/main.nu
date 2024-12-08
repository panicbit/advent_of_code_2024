let instructions = open input.txt
    | parse -r `(?P<op>mul|don't|do)\((?:(?P<a>\d+),(?P<b>\d+))?\)`
    | update a { try { into int } }
    | update b { try { into int } }

mut enabled = true
mut sum = 0

for instruction in $instructions {
    match $instruction.op {
        "mul" if $enabled => { $sum += $instruction.a * $instruction.b }
        "do" => { $enabled = true }
        "don't" => { $enabled = false }
    }
}

$sum