import Client, { connect } from "../../deps.ts";

export enum Job {
  build = "build",
  releaseUpload = "release_upload",
}

export const exclude = [];

export const build = async (src = ".") => {
  await connect(async (client: Client) => {
    const context = client.host().directory(src);
    const ctr = client
      .pipeline(Job.build)
      .container()
      .from("rust:latest")
      .withDirectory("/app", context, { exclude })
      .withWorkdir("/app")
      .withMountedCache("/app/target", client.cacheVolume("target"))
      .withMountedCache("/root/cargo/registry", client.cacheVolume("registry"))
      .withMountedCache("/assets", client.cacheVolume("gh-release-assets"))
      .withExec([
        "cargo",
        "build",
        "--release",
        "--target",
        "x86_64-unknown-linux-gnu",
      ])
      .withExec([
        "tar",
        "czvf",
        `/assets/hello_${
          Deno.env.get("TAG") || ""
        }_x86_64-unknown-linux-gnu.tar.gz`,
        "target/x86_64-unknown-linux-gnu/release/github-release-demo",
      ])
      .withExec([
        "sh",
        "-c",
        `shasum -a 256 /assets/hello_${
          Deno.env.get("TAG") || ""
        }_x86_64-unknown-linux-gnu.tar.gz > /assets/hello_${
          Deno.env.get("TAG") || ""
        }_x86_64-unknown-linux-gnu.tar.gz.sha256`,
      ]);

    await ctr.stdout();
  });
  return "Done";
};

export const releaseUpload = async (src = ".", tag?: string, file?: string) => {
  await connect(async (client: Client) => {
    const TAG = Deno.env.get("TAG") || tag || "latest";
    const FILE = Deno.env.get("FILE") || file!;
    const context = client.host().directory(src);
    const ctr = client
      .pipeline(Job.releaseUpload)
      .container()
      .from("pkgxdev/pkgx:latest")
      .withExec(["apt-get", "update"])
      .withExec(["apt-get", "install", "-y", "ca-certificates"])
      .withExec(["pkgx", "install", "gh"])
      .withMountedCache("/assets", client.cacheVolume("gh-release-assets"))
      .withDirectory("/app", context)
      .withWorkdir("/app")
      .withEnvVariable("GH_TOKEN", Deno.env.get("GH_TOKEN") || "")
      .withExec(["gh", "release", "upload", TAG, FILE]);

    await ctr.stdout();
  });

  return "Done";
};

export type JobExec = (
  src?: string,
  tag?: string,
  file?: string
) => Promise<string>;

export const runnableJobs: Record<Job, JobExec> = {
  [Job.build]: build,
  [Job.releaseUpload]: releaseUpload,
};

export const jobDescriptions: Record<Job, string> = {
  [Job.build]: "Compile the project",
  [Job.releaseUpload]: "Upload asset files to a GitHub Release",
};
