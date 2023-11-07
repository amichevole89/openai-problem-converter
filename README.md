# openai_problem_converter

> Python -> TypeScript/JavaScript Problem Converter

The Python to TypeScript/JavaScript Problem Converter is a Rust command-line tool that utilizes the OpenAI API to convert Python problems and solutions into TypeScript or JavaScript based on user preference. It automates the process of translating Python practice problems and solutions to the selected target language, enabling you to convert Python problems and solutions into TypeScript or JavaScript.

> Note: The conversion is not perfect, and it may not work for all Python code. It is intended to be used as a starting point for your TypeScript/JavaScript code, which you can then modify and improve as needed.

---

> Additional work will be done experimenting with openai's API to improve the accuracy of the conversion process.

---

## Table of Contents

- [openai\_problem\_converter](#openai_problem_converter)
  - [Table of Contents](#table-of-contents)
  - [How It Works](#how-it-works)
  - [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Notes](#notes)
  - [Future Development and Potential Enhancements](#future-development-and-potential-enhancements)

---

## How It Works

The converter is designed to provide a seamless and efficient conversion process. Here's an overview of the key functionalities:

<details>
<summary><code>ai.rs</code></summary>

The `ai.rs` module contains a function `send_ai_request` responsible for making requests to the OpenAI API and converting Python code to TypeScript or JavaScript based on user choice. This function takes the following inputs:

- `python_code`: The Python code that needs to be converted.
- `target_language`: The desired target language, which can be either "TypeScript" or "JavaScript".

The function constructs a request to the OpenAI API with the provided parameters, including the `python_code` as the user input. The API response is processed to extract the converted code in the specified target language.

</details>

<details>
<summary><code>file_processing.rs</code></summary>

The `file_processing.rs` module contains functions related to file processing and conversion. It includes the following functions:

- `process_files`: This function is responsible for processing the files in the given input folder and converting Python code to the specified target language using the OpenAI API. It takes the following inputs:

  - `input_folder`: The path to the input folder containing Python files to be converted.
  - `output_folders`: A HashMap containing mappings of target languages ("TypeScript" or "JavaScript") to their respective output folders.
  - `prefix`: A prefix string used to identify the file index.
  - `target_language`: The desired target language, which can be either "TypeScript" or "JavaScript".

  The function reads the Python files from the input folder, converts each file using the `send_ai_request` function from the `ai.rs` module, and writes the converted code to the corresponding output folders based on the target language.

</details>

<details><summary><code>input_utility.rs</code></summary>

The `input_utility.rs` module provides utility functions for handling user input. It includes the following functions:

- `get_language_input`: This function prompts the user to select the target language ("TypeScript" or "JavaScript") and returns the selected language as a `TargetLanguage` enum variant.

- `get_user_choice`: This function prompts the user for a choice, with the option to provide a default value. It returns the user's choice as a string.

</details>

<details>
<summary><code>main.rs</code></summary>

The `main.rs` file serves as the entry point to the application. It uses the functionalities provided by the other modules to perform the Python to TypeScript/JavaScript code conversion. Here's how the application works:

- It prompts you to enter the paths to the "problems" and "solutions" folders, which contain the Python files to be converted.
- You then select the target language by choosing either "TypeScript" or "JavaScript".
- The application processes the Python files in the input folders, converts them to the chosen language using the `process_files` function from the `file_processing.rs` module, and writes the converted code to the output folders.

</details>

---

## Getting Started

To use the Python to TypeScript/JavaScript Code Converter, follow these steps:

1. Set up your OpenAI API key by adding it to a `.env` file in the project root:

   ```dotenv
   OPENAI_API_KEY=your_openai_api_key_here
   OPENAI_PROMPT=your_openai_prompt_here
   ```

2. By default, the converter will look for a folder called `problems` and `solutions` in the root directory. The `problems` folder should contain the Python files to be converted, and the `solutions` folder should contain the Python files to be converted.

   Here's an example of the folder structure:

   ```bash
   .
   ├── problems
   │   ├── problem_001.py
   │   ├── problem_002.py
   │   └── problem_003.py
   └── solutions
       ├── solution_001.py
       ├── solution_002.py
       └── solution_003.py
   ```

3. Run the application using `cargo run`.

4. When prompted, input the paths to your "problems" and "solutions" folders (relative to the project root). If you've supplied the program with `problems/` and `solutions/` within the root directory, you do not need to specify their paths. Just press enter.

5. Use your arrow keys to select the target language `TypeScript` or `JavaScript` then press enter.

6. The application will convert the Python files in the specified input folders and write the results to the corresponding output folders.

---

## Dependencies

The converter leverages the following external crates from the Rust ecosystem:

- `reqwest` for making HTTP requests to the OpenAI API.
- `serde_json` for parsing and handling JSON responses.
- `log` and `simple_logger` for logging purposes.
- `colored` for colorful terminal output.
- `dialoguer` for user input prompts.

---

## Notes

- Ensure that you have a stable internet connection while using the converter to access the OpenAI API.
- The converter assumes that you have provided valid Python code as input. It may not handle incomplete or syntactically incorrect code gracefully.

---

## Future Development and Potential Enhancements

Exciting ideas for the future development of the converter to make it even more powerful and user-friendly. Here are some potential enhancements I've considered:

1. **Continuous Conversation**: Optimize the cost of this process by training the conversation with more examples and information, and avoid sending the prompt on every request. This could lead to cost reduction and better performance.

2. **Reducing API Calls**: Optimize the process by processing files in pairs (problem-solution) and generating both the problem and solution in a single API call, reducing the number of API requests.

3. **Error Handling**: Enhance error handling throughout the program to gracefully handle file reading errors, API call errors, and file writing errors. Providing informative error messages will improve user experience and make troubleshooting easier.

4. **Optimize the OpenAI Prompt**: Continuously iterate on the OpenAI prompt to make it as clear and concise as possible. Fine-tuning the prompt can lead to improved accuracy in code conversion.
