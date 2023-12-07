#!/usr/bin/env python
# Script to publish for release on platforms.
import subprocess
import os
import zipfile
import shutil

# Clear previous dist files
try:
  shutil.rmtree(os.path.abspath("dist"))
except:
  pass

try:
  os.mkdir("dist", 0o666)
except:
  pass
  
executable_name = "jot"
static_files = [os.path.abspath("README.md"), os.path.abspath("LICENSE"), os.path.abspath("CHANGELOG.md")]
target_list = ['x86_64-unknown-linux-gnu', 'x86_64-pc-windows-gnu']
zipfile_list = []
# Publish release for platform
for target in target_list:
  result = subprocess.run(f"cross build --target {target}", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
  
  out = result.stdout.decode('utf-8').rstrip()
  if out:
    print(out)
  err = result.stderr.decode('utf-8').rstrip()
  if err:
    print(err)
    
  zip_file = os.path.abspath(os.path.join("dist", f"{executable_name}-{target}.zip"))
  if os.path.isfile(zip_file):
    os.remove(zip_file)
  
  with zipfile.ZipFile(zip_file, "w", zipfile.ZIP_DEFLATED) as archive:
    for s in static_files:
      archive.write(s, os.path.basename(s))
      
    try: 
      file_path = os.path.abspath(os.path.join("target", target, "debug", "jot"))
      archive.write(file_path, os.path.basename(file_path))
    except:
      pass
    try:
      file_path = os.path.abspath(os.path.join("target", target, "debug", "jot.exe"))
      archive.write(file_path, os.path.basename(file_path))
    except:
      pass
  zipfile_list.append(zip_file)
  