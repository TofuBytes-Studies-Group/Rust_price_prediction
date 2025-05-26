# Skyrin weapon generator and price prediction

## This is part 2 of the ML and AI exam.

This project generates lore-friendly Skyrim-style weapons using a locally hosted AI model, predicts their prices using a trained machine learning model, 
and stores the final result in a MongoDB database. It combines Rust for orchestration and backend logic, Python for model inference and AI communication, 
and Ollama for local AI generation.

- We are using the jupyter notebook to clean the dataset and learn the models how to predict the prices
- After that, we use the codebase to give a name for the weapon in the terminal, and the AI then responds with a full name, damage, weight, upgrade, perk and type.
- Lastly, we take the AI generated answer, and put it into a MongoDB Database.´

We are connecting to the local AI using the API url http://localhost:11434/api/generate


## Setup

### 1. Ollama

Download ollama and make sure to have downloaded the AI.
- ollama pull hf.co/DavidAU/Gemma-The-Writer-Mighty-Sword-9B-GGUF:Q2_K

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


## Description

We have two models that we trained: 
- model.pkl and ordinal_encoder.pkl. 

The model is our model we trained to predict the prices and the ordinal_encoder is used to convert categorical text labels into numerical values, to use for the prediction.

***model_predictor.py:*** This function loads a trained ML model and encoder, preprocesses input weapon stats (cleaning strings and ensuring consistent formatting), and predicts the weapon's price while maintaining the same feature structure used during training.

***ai_calls.py:*** This code uses an AI API to generate Skyrim-style weapon names and detailed stats (Damage, Weight, Upgrade, etc.) from a base name.

- *generate_weapon_name():* Creates a lore-friendly weapon name (e.g., "Jamie’s Vengeance") from a base (e.g., "Jamie").

- *generate_weapon():* Produces realistic stats (e.g., Damage: 25, Weight: 12, Type: Sword) for the generated name, parsed into a structured format.

***main.rs:*** This rust code generates a weapon based on the AI call, it then predicts the price, creates the final weapon and adds it to the database.
