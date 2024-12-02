# Advent of Code 2024 - Days 1, 2 & 3 Solutions

## Day 1: Distance and Similarity Calculation

### Algorithm Overview
- **Input:** Two lists of integers, paired line by line
- **Part 1:** Calculate minimum total distance between sorted pairs
- **Part 2:** Calculate similarity score based on frequency matching

### Solution Approach
1. **Part 1 - Total Distance:**
   - Create two vectors: left and right from input pairs
   - Sort both vectors independently
   - Zip sorted vectors and calculate absolute differences
   - Sum all differences for total distance

2. **Part 2 - Similarity Score:**
   - Create frequency map of right list numbers
   - For each number in left list:
     - Multiply by its frequency in right list
     - Sum all products for final score

### Complexity
- **Time:** O(n log n) due to sorting
- **Space:** O(n) for storing lists and hash map
- **n:** Number of input pairs

## Day 2: Red-Nosed Reports Analysis

### Algorithm Overview
- **Input:** Lists of integer sequences
- **Part 1:** Check if sequences are strictly increasing/decreasing with constraints
- **Part 2:** Allow removal of one number to make sequence valid

### Solution Approach
1. **Part 1 - Safety Check:**
   - Validate adjacent differences (1-3 range)
   - Track if sequence is strictly increasing or decreasing
   - Sequence is safe if:
     - All differences are 1-3
     - Direction is consistent (all increasing or all decreasing)

2. **Part 2 - Problem Dampener:**
   - First try original safety check
   - If unsafe, try removing each number once:
     - Create new sequence without current number
     - Run safety check on modified sequence
     - If any removal creates safe sequence, count as safe

### Complexity
- **Time:** O(n²) in worst case for Part 2
- **Space:** O(n) for storing sequences
- **n:** Length of each sequence

## Day 3: Mull It Over

### Algorithm Overview
- **Input:** A corrupted memory string containing `mul(X,Y)` instructions and conditional `do()`/`don't()` commands
- **Part 1:** Identify and sum all valid `mul(X,Y)` multiplication results
- **Part 2:** Handle `do()` and `don't()` instructions to enable or disable `mul()` operations and sum only the enabled multiplications

### Solution Approach
1. **Part 1 - Summing Multiplications:**
   - Use regular expressions to find all valid `mul(X,Y)` patterns where `X` and `Y` are 1-3 digit numbers
   - Extract the numerical values of `X` and `Y`
   - Multiply `X` by `Y` for each valid instruction
   - Sum all multiplication results to obtain the final total

2. **Part 2 - Conditional Multiplications:**
   - Initialize a flag to keep track of whether `mul()` operations are enabled (starts as enabled)
   - Iterate through the corrupted memory string sequentially
   - When a `do()` instruction is encountered, enable `mul()` operations
   - When a `don't()` instruction is encountered, disable `mul()` operations
   - For each `mul(X,Y)` instruction:
     - If `mul()` is enabled, perform the multiplication and add to the sum
     - If disabled, ignore the multiplication
   - Continue processing until the end of the input string
   - Sum all enabled multiplication results for the final total

### Complexity
- **Time:** O(n) for both parts, where `n` is the length of the input string
- **Space:** O(1) for Part 1 and Part 2 (excluding input storage)
- **n:** Length of the corrupted memory string
