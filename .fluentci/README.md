# Github Pipeline

[![fluentci pipeline](https://img.shields.io/badge/dynamic/json?label=pkg.fluentci.io&labelColor=%23000&color=%23460cf1&url=https%3A%2F%2Fapi.fluentci.io%2Fv1%2Fpipeline%2Fgithub_pipeline&query=%24.version)](https://pkg.fluentci.io/github_pipeline)
![deno compatibility](https://shield.deno.dev/deno/^1.37)
[![](https://img.shields.io/codecov/c/gh/fluent-ci-templates/github-pipeline)](https://codecov.io/gh/fluent-ci-templates/github-pipeline)

A ready-to-use CI/CD Pipeline for uploading assets to github releases.

## 🚀 Usage

Run the following command:

```bash
fluentci run github_pipeline
```

## Environment Variables

| Variable              | Description                   |
|-----------------------|-------------------------------|
| GH_TOKEN              | Github Access Token           |


## Jobs

| Job            | Description                                                |
|----------------|------------------------------------------------------------|
| release_upload | Uploads a file to a github release                         |

```graphql
    releaseUpload(file: String!, src: String!, tag: String!): String
```

## Programmatic usage

You can also use this pipeline programmatically:

```typescript
import { releaseUpload } from "https://pkg.fluentci.io/github_pipeline@v0.1.0/mod.ts";

await releaseUpload();
```
