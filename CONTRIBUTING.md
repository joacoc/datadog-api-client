# Contributing

In this guide, you will get an overview of the contribution workflow, starting from opening an issue, creating a PR, reviewing it, and merging the PR.

## New Contributor Guide

To gain an overview of the project, please read the [README](./README.md) file.

## Getting Started

Clone the repository and ensure that you have nightly set up as the default compiler:
```bash
rustup override set nightly
```

## Developer Workflow

1. Create a new branch.
2. Make your changes and tests.
3. Successfully run `cargo test` and `cargo clippy`.
4. Request a review for your PR.
5. Merge after approval.

## Testing

The client tests the endpoints by mocking Datadog's API examples from their documentation. For more information about these examples, refer to [Datadog's API documentation](https://docs.datadoghq.com/api/latest/).

## Cutting a Release
[TBD]

## References

* [Datadog's API Docs](https://docs.datadoghq.com/api/latest/)
