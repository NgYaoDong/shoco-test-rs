import os
import sys

def read_file(file):
  with open(file, 'r') as f:
    return f.read()

TRANS_PATH = os.getcwd()
PROMPT_PATH = os.path.join(TRANS_PATH, 'TRANS_TMPL.md')
PROMPT_FILE_CONTENT = read_file(PROMPT_PATH)

# this program convert PROMPT_FILE_CONTENT into a sequence of prompts for GPT4.
# The file content is a markdown file with the following structure:
"""
``````
<INITIAL_PROMPT_TEMPLATE>
``````
``````
<CONTINUE_PROMPT_TEMPLATE>
``````

## Full Program

<FULL_PROGRAM_CONTENT>
"""

def parse_prompt_file_content(prompt_file_content):
  initial_prompt = None
  continue_prompt = None
  full_program_content = None
  part_contents = []
  # split before ## Full Program
  full_program_split = prompt_file_content.split('\n## Full Program')
  assert len(full_program_split) == 2, f'Expecting 2 parts, but got {len(full_program_split)}'
  splits = full_program_split[0].split('``````')
  assert len(splits) == 5, f'Expecting 5 parts, but got {len(splits)}'
  initial_prompt = splits[1]
  continue_prompt = splits[3]
  full_program_content = full_program_split[1].strip()
  assert full_program_content.startswith("```c\n")
  assert full_program_content.endswith("\n```")
  full_program_content = full_program_content[5:-4]
  # assert that there is no '```c` inside full_program_content
  assert '```c' not in full_program_content
  # split full_program_content into parts according to `///// PART_SPLIT /////`
  part_contents = full_program_content.split('\n///// PART_SPLIT /////\n')
  # for each part, assert no `PART_SPLIT` inside
  for part_content in part_contents:
    assert 'PART_SPLIT' not in part_content
  full_program_content = "\n".join(part_contents)
  return initial_prompt, continue_prompt, full_program_content, part_contents


initial_prompt, continue_prompt, full_program_content, part_contents = parse_prompt_file_content(PROMPT_FILE_CONTENT)
print("len(initial_prompt):", len(initial_prompt))
print("len(continue_prompt):", len(continue_prompt))
print("len(full_program_content):", len(full_program_content))
for idx, part_content in enumerate(part_contents):
  print(f'{idx}: {len(part_content)}')

# Save initial_prompt (instantiated) into file TRANS.step0.md
# Save continue_prompts using part_contents into TRANS.stepX.md

def save_to_file(file, content):
  with open(file, 'w') as f:
    f.write(content)

def apply_template(template, **kwargs):
  for k, v in kwargs.items():
    template = template.replace(f'{{TPL:{k}}}', str(v))
  assert "{TPL:" not in template, f"Unfilled template: {template}"
  return template

def step_path(step):
  return os.path.join(TRANS_PATH, "TRANS.input" + str(step) + ".md")

save_to_file(step_path(0), apply_template(initial_prompt, 
  FULL_PROGRAM_CONTENT=full_program_content,
  PART_IDX=0,
  PART_CONTENT=part_contents[0]))

for idx, part_content in enumerate(part_contents[1:]):
  save_to_file(step_path(idx + 1), apply_template(continue_prompt, 
    FULL_PROGRAM_CONTENT=full_program_content,
    PART_IDX=idx + 1,
    PART_CONTENT=part_content))

