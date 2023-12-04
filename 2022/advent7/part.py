class Directory:
    def __init__(self):
        self.size = 0
        self.subdirs = set()

dirs = {"/": Directory()}
current_dir = "/"

def parse_cmd(tokens):
    global current_dir, dirs
    if tokens[1] == "cd":
        if tokens[2] == '/':
            current_dir = '/'
            print("Moving to root", current_dir)
        elif tokens[2] == '..':
            current_dir = current_dir[:(current_dir[:-1].rfind('/')+1)]
            print("Moving back", current_dir)
        else:
            dirs.setdefault(current_dir, Directory()).subdirs.add(tokens[2])
            current_dir += tokens[2] + '/'
            print("Moving to", tokens[2], current_dir)
    else:
        assert tokens[1] == "ls"
        print("lsing")

for l in open('input.txt'):
    tokens = l.strip().split(' ')
    if tokens[0] == '$':
        parse_cmd(tokens)
    elif tokens[0] == "dir":
        dirs.setdefault(current_dir, Directory()).subdirs.add(tokens[1])
    else:
        dirs.setdefault(current_dir, Directory()).size += int(tokens[0])

# print(dirs)
def blah(d):
    global dirs
    for dd in dirs[d].subdirs:
        n = d + dd + '/'
        if n in dirs:
            dirs[d].size += blah(n)
    dirs[d].subdirs.clear()
    return dirs[d].size

blah("/")
t = 0

for d in dirs:
    print(d, dirs[d].size)
    if dirs[d].size <= 100000:
        t += dirs[d].size

print(t)

total = dirs['/'].size
need = 70000000-30000000
print(sorted([(d.size, i) for i, d in dirs.items() if d.size > (total - need)])[0])
