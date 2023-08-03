import re

# Using Python because rust regex doesn't allow backreferences
count = 0
for line in open("input.txt"):
    if re.search(r"(..).*\1", line) and re.search(r"(.).\1", line):
        print(line.strip())
        count += 1
print(count)
