# _VSCode_Rust  (Learning on Runoob)

[https](https://github.com/OuyangXuanyu/_VSCode_Rust.git)

or

[git](git@github.com:OuyangXuanyu/_VSCode_Rust.git)

# Actually

How to use git to push you project on your github account?

Step 1 install git and bind your github account email and password

Step 2 Generate a key so that the local git could have power to access your github

Step 3 Make a repository, give a name and record the https or git@github

Step 4 To indentify your local explorer you wanna upload, do these commends in you bash(cmd/powershell in Windows or sh in Linux)

```bash
git init  # init you explorer
git add .  # add files you wanna upload, '.' means all your files
git commit -m "your comment in this version"  # make it ready for uploading
git remote add origin https://github.com/your-username/your-repo.git  # add location website
git push origin master  # do the final step
```

but how to use rust to generate an exe document
so this is a proj that i used for learning rust



