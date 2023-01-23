# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2023-01-23

### Bug fixes

- Reword description ([6c0be77](6c0be77f4c6ab2043b4e840268902dbdb466e81e))

## [0.2.0] - 2023-01-16

### Features

- Accept reading input from a file ([53e34ff](53e34ff2a3d59ce8914495deb42309884a9e7304))
- Get line and column of error position ([5eca6e8](5eca6e8e0218d0d97c4c73841284d1e3ec6d5bde))
- Print fancy error message on error ([4adb459](4adb459fc5d0bd3b1a1ce1e9a45b9ad129fd459f))
- Better error message ([653f170](653f170a0e1bc0293a5ddb91b6efb14070997d90))

### Bug fixes

- Respect user's color choice ([d011f5e](d011f5e0281a365968d2a2cad8bdbce19eef4c5d))
- Exit code should be non-zero if any files had errors or were not formatted ([00a1bd1](00a1bd1c783c73471db854df42c1dd9fde47acb5))
- Change the input files as direct argument ([cacfbdc](cacfbdc1d3e0a877f1090c6dce61078e26227d25))
  - **BREAKING!** ⚠️ : this change the previous argument, where `language` is
    the direct argument. Now it is the file input. I see this pattern is
    more common than the previous design.
- Return error code on formatting failure ([7c82da2](7c82da2517fbb08edd615a772745fbf15118c838))
- Remove the feature flag ([5d36ac8](5d36ac8706dfed82c75c7d81fc146b66dadc9ad4))
- Migrate to clap derive API ([65dad31](65dad31781be2cf8d8c6759451c6a4397af414e7))
- Exit code as enum is more readable than integer ([5f8e74f](5f8e74fe258fdb09e7f9516e11ab857599de0144))

## [0.1.5] - 2022-07-18

### Features

- Allow whitespace after backtick ([0e3dbfc](0e3dbfc5dbb13c65b93aee86ecacfdcf821aa6df))
- Respect color option ([d048148](d0481483918bcddb1fd504ee9886fdaccc72b9cf))

## [0.1.4] - 2022-07-18

### Features

- Configuration for language formatter ([bc37b30](bc37b300a8aafbdddde66247c503fb49fe19fdbe))

## [0.1.3] - 2022-07-17

### Bug fixes

- Don't change invalid code ([41e4bd4](41e4bd4ced18bd6deae8194ab74f9aa00ab1a701))
- Display file name on IO error ([120ca53](120ca537f3b8d43fb8bf317622ae87244f3b605e))

## [0.1.2] - 2022-07-17

### Features

- Support multiple code block in a file ([dfc2855](dfc28551de5102baccb39d073a0d472e61055366))
- Check mode ([8b1de34](8b1de34633a374c6475f6f3f8a82b06fb43585ee))

## [0.1.0] - 2022-07-14

### Features

- Prevent app crash on file with no code block ([eb0882d](eb0882d6ca70ce4d1323e96c592243e66e9d4670))
- Multiple files as input ([ce9a158](ce9a158867a536a512d57f1a1bdf28dc5b4da28d))
- Optional dependencies ([4335182](4335182f54d4f1f33a81bd90091c3d787f823d61))
- Language choice ([4021fc1](4021fc190bf8c9a41bd20137cecc84bc1e83640c))
- Integrate stylua ([6a5b27e](6a5b27e7a35faeea3809d7eab96c5f97c0c7b95c))
