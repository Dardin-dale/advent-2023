lines = []
with open("input.txt", "r") as input:
    lines = input.readlines();

# Part 2 Only 
def replaceWords(line):
    wordNums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]
    for i in range(len(wordNums)):
        if wordNums[i] in line:
            line = line.replace(wordNums[i], wordNums[i] + str(i + 1) + wordNums[i])
    return line
###

def findFirstDigit(line):
    digits = "0123456789"
    for char in line:
        if char in digits:
            return char

out = 0
for line in lines:
    line = replaceWords(line) # Part 2 Only
    first = findFirstDigit(line)
    last = findFirstDigit(line[::-1])
    out += int(str(first) + str(last))
print(out)
