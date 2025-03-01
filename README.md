# FibBot Custom Action Test Project

![GitHub Actions Status](https://github.com/bansikah22/test-fibbot/actions/workflows/test.yml/badge.svg)
![GitHub Issues](https://img.shields.io/github/issues/bansikah22/test-fibbot?color=red)
![Open Source](https://img.shields.io/github/license/bansikah22/test-fibbot?color=green)

This is a test project to validate the functionality of the FibBot GitHub Action. It demonstrates how to use the FibBot action from the `bansikah22/fibbot` repository within a separate project.

## Setup

1. **Fork the FibBot repository**:
   - Fork the [FibBot repository](https://github.com/bansikah22/test-fibbot) to your GitHub account if you haven't already.

2. **Clone the test project**:
   - Clone this test repository to your local machine or GitHub account.

3. **Create a Pull Request**:
   - Modify the `src/some_test_file.rs` or add a new file to simulate a change.
   - Create a pull request in this repository. The FibBot action will be triggered when the pull request is created, and it will calculate the Fibonacci sequence and post a comment with the result.

4. **Check the Action Output**:
   - After the PR is created, the action will run and you will see the output in the pull request conversation.
   - The FibBot action will output Fibonacci numbers based on the numbers in the PR description.

## Workflow

- The `test-fibbot.yml` workflow runs whenever a pull request is opened, synchronized, or reopened.
- The workflow triggers the `fibbot` action and passes inputs like `enable_fib` and `max_threshold` to it.

## Expected Behavior

When the pull request is opened, the FibBot action should:
- Extract numbers from the PR description.
- Compute the Fibonacci sequence for those numbers.
- Post a comment with the Fibonacci results.

## License

This project is licensed under the MIT License.

---

### Open Source

- This repository is open-source and licensed under the [MIT License](LICENSE).
- Contributions are welcome! Please feel free to open issues or submit pull requests to improve this project.
