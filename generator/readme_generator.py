from pathlib import Path
from enum import Enum
from codecs import open
from datetime import datetime
from os.path import getmtime

class Difficulty(Enum):
    Easy = "Easy"
    Medium = "Medium"
    Hard = "Hard"


class Problem:
    number: int
    title: str
    url: str
    difficulty: Difficulty
    src: str

    def __init__(self, number, title, url, difficulty, src):
        self.number = number
        self.title = title
        self.url = url
        self.difficulty = difficulty.upper()
        self.src = f"{src}"

    def __repr__(self):
        return f"<Problem number={self.number} title={self.title} url={self.url} diff={self.difficulty} src={self.src}>"

    @property
    def table_line(self):
        return f"| {self.number} | {self.title} | [SOLUTION]({self.src}) && [LEETCODE]({self.url})| {self.difficulty} |"

paths = Path("./").glob("**/**/*.rs")

problems = []

for path in paths:

    number = None
    title = None
    url = None
    difficulty = None
    with open(str(path), encoding="utf8") as file:
        lines = file.readlines
        for line in lines():
            if line.startswith("/// @number"):
                number = int(line[11:].strip())
            if line.startswith("/// @title"):
                title = line[10:].strip()
            if line.startswith("/// @url"):
                url = line[8:].strip()
            if line.startswith("/// @difficulty"):
                difficulty = line[15:].strip()

        if number is not None:

            problems.append(Problem(number, title, url, difficulty, path))

problems = sorted(problems, key=lambda i: i.number)

def table(problems):
    header = [
        '| ID   | TITLE | LINK | DIFFICULTY |',
        '| ---- | ----- | ---- | ---------- |'
    ]

    ret = []
    ret.extend(header)
    ret.extend([problem.table_line for problem in problems])
    return '\n'.join(ret)


now = datetime.now()

with open("./generator/readme.template") as file:
    content = file.read()

    content = content.replace(r"{time}", now.strftime("%Y-%m-%d %H:%M:%S"))
    content = content.replace(r"{problems}", table(problems))
    content = content.replace(r"{count}", str(len(problems)))

    with open("./readme.md", mode="w") as readme:
        readme.write(content)

print("done")