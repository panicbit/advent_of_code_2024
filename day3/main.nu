open input.txt | parse -r 'mul\((?P<a>\d+),(?P<b>\d+)\)' | update cells { into int } | each {$in.a * $in.b} | math sum
