## Setup

### 1. Ollama

Download ollama and make sure to have downloaded the AI.
- ollama pull DavidAU/Gemma-The-Writer-Mighty-Sword-9B-GGUF

Check if it is downloaded:
- ollama list

### 2. Database setup

Check main.rs and add your database connection
- let client_options = ClientOptions::parse("db connection")...
- let db = client.database("your db name")...
- let coll = db.collections::<Weapon>("your collection")...


### 3. Python setup in RustRover

Check if it is the correct version of python you are running. 
We are working with Python 3.13.

Go to File -> Settings -> Build, Execution, Deploy -> Python Interpreter and then find your Python ( Python 3.13 exe file ) and add it as interpreter
- Check if it works by running the code

Everything should work after this.
