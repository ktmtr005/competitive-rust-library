#!/usr/bin/python3

import sys
import getopt
import tempfile
import subprocess
import os

usage = '''Usage: expander.py [options] <output modules>
Output Modules:
    io
    math
    num_trait

You can select multiple modules for <output modules>
    e.g.) expand.py io math

Options:
    -a --all        import all modules
    -h --help       print help
    -o file
    --output file   output file
'''
output_header = '// https://github.com/ktmtr005/competitive-rust-library\n'
opt_list = ['help', 'all', 'output=']
output_list_all = ('io', 'math', 'num_trait')
dependency_list = {'math': ('num_trait',)}
src_path = 'src/'
output_path = None

def output_file(modulename):
    global src_path

    res = []
    res.append('pub mod {} {{'.format(modulename))
    files = os.listdir(src_path+modulename)
    for file in files:
        with open(src_path+modulename+'/'+file, 'r', encoding='utf-8', newline='') as f:
            for line in f:
                if line.startswith('#!'):
                    continue
                if line.startswith('#[cfg(test)]'):
                    break
                if line.startswith('use crate::'):
                    line_split = line.split('::')
                    res.append('use super::'+line_split[1]+'::'+line_split[-1])
                    continue
                res.append(line.rstrip())

    res.append('}')
    return res

try:
    opts, args = getopt.getopt(sys.argv[1:], 'aho:', opt_list)
except getopt.GetoptError as e:
    print(e)
    print(usage)
    sys.exit(2)

if len(opts) == 0 and len(args) == 0:
    print(usage)
    sys.exit(0)

for o, v in opts:
    if o == '--help' or o == '-h':
        print(usage)
        sys.exit(0)
    elif o == '--all' or o == '-a':
        args = list(output_list_all)
    elif o == '--output' or o == '-o':
        output_path = v

output_list = set()
while len(args) != 0:
    pop = args.pop()
    if pop not in output_list_all:
        print('invalid args: {}'.format(pop))
        print(usage)
        sys.exit(2)
    output_list.add(pop)
    if pop in dependency_list:
        for d in dependency_list[pop]:
            args.append(d)

output_list = list(output_list)
output_list.sort()

output_data = []
output_data.append('#[allow(dead_code)]')
output_data.append('pub mod lib {')
for i in output_list:
    buf = output_file(i)
    output_data.extend(buf)
output_data.append('}')

# rustfmt
with tempfile.TemporaryDirectory() as temp_dir:
    temp_file = temp_dir + '/output.rs'
    with open(temp_file, 'w', encoding='utf-8', newline='') as f:
        print(output_header, file=f)
        for i in output_data:
            print(i, file=f)
    output_data = subprocess.run(["rustfmt", temp_file], check=True)
    with open(temp_file, 'r', encoding='utf-8', newline='') as f:
        wf = open(output_path, 'w', encoding='utf-8', newline='') if output_path is not None else sys.stdout
        for line in f:
            print(line, end='', file=wf)
        if output_path is not None:
            wf.close()
