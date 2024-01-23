import {
  assertEquals,
  assertObjectMatch,
} from "https://deno.land/std@0.212.0/assert/mod.ts";
import { getInfo } from "./function.ts";

Deno.test("動画の情報を取得できる", async () => {
  const result = await getInfo(
    "https://submarin.online/files/3e327bd6-8a90-49d0-8aed-1864c20e5209",
  );

  assertEquals(result.result, "ok");
  assertObjectMatch(result, {
    result: "ok",
    fileInfo: {
      name: "mov,mp4,m4a,3gp,3g2,mj2",
      bitRate: 56565,
      duration: 3750000,
      url: "/3e327bd6-8a90-49d0-8aed-1864c20e5209",
      nbStreams: 1,
      flags: 2097152,
      nbChapters: 0,
      streams: [
        {
          id: 1,
          startTime: 0,
          duration: 57600,
          codecType: 0,
          codecName: "avc1",
          format: "yuv420p",
          bitRate: 48947,
          profile: "High",
          level: 13,
          width: 468,
          height: 82,
          channels: 0,
          sampleRate: 0,
          frameSize: 0,
          tags: [],
        },
      ],
    },
  });
});

Deno.test("URLでない文字列を渡すとエラーになる", async () => {
  const result = await getInfo("hoge");

  assertEquals(result.result, "err");
  assertObjectMatch(result, { result: "err", error: "Invalid input: not URL" });
});

Deno.test("動画でないファイルを渡すとエラーになる", async () => {
  const result = await getInfo(
    "https://submarin.online/files",
  );

  assertEquals(result.result, "err");
  assertObjectMatch(result, {
    result: "err",
    error: "Invalid input: not video",
  });
});
