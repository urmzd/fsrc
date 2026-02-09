# Changelog

## 1.5.0 (2026-02-09)

### Features

- add floating major-version tag and multi-comment-style integration tests ([f5c79d1](https://github.com/urmzd/embed-it/commit/f5c79d1469f50c73fcaaca1e957af903bb2b0354))

### Bug Fixes

- skip directives inside fenced code blocks and fix stale example paths ([c4c8ca6](https://github.com/urmzd/embed-it/commit/c4c8ca67d0319aa263eeec97ff1c42a18ccf3e71))
- fix CI integration test, add cargo publish, clean up examples and README ([9464e35](https://github.com/urmzd/embed-it/commit/9464e35b8a0d2bafbba5dd90e22b3cfdabcd8cda))

### Contributors

- @urmzd


## 2.0.0 (unreleased)

### Breaking Changes

- **Rename**: `embed-md` is now `embed-it` — the tool is no longer markdown-only.
- **Marker syntax**: `<!-- embedmd src="..." -->` / `<!-- /embedmd -->` is now `embed-it src="..."` / `/embed-it` (comment-agnostic).
- **Raw by default**: Content is inserted raw. Use `fence`, `fence="auto"`, or `fence="lang"` to wrap in code fences.
- **Action input**: `markdown-files` is now `files`.

### Features

- **Any file type**: Embed into markdown, YAML, Python, Rust, shell scripts, or any text file.
- **`fence` attribute**: Opt-in code fencing with auto-detection or explicit language.
- **Comment-agnostic markers**: Works with `//`, `#`, `/* */`, `--`, `<!-- -->`, or any comment style.

## 1.4.0 (2026-02-08)

### Features

- rewrite embed-md in Rust ([2a0dfb7](https://github.com/urmzd/embed-md/commit/2a0dfb710d69d941dc1d2b0eb30a3f81c152d4f0))

### Bug Fixes

- apply cargo fmt formatting ([b705643](https://github.com/urmzd/embed-md/commit/b705643773d7b26a80cb980741233a3ccded5299))
- **examples**: remove embedded content to show clean embed markers ([681e522](https://github.com/urmzd/embed-md/commit/681e5226a1b7b6259cb3f2a1b6662b3583b42074))
- **ci**: use urmzd/semantic-release action instead of calling sr directly ([341fa66](https://github.com/urmzd/embed-md/commit/341fa6695f703c2b082ffafc1d4985268067de6b))
- **ci**: assert no drift instead of asserting changes in example ([698f7b3](https://github.com/urmzd/embed-md/commit/698f7b3888ec47e3336296ca519e22983ead9a7a))
- use dynamic code fence length to avoid nested fence collisions ([27bdaeb](https://github.com/urmzd/embed-md/commit/27bdaeb1e03728b0b73e59a6a14bbde740b0e679))
- **ci**: run CLI directly instead of invoking composite action ([57857ce](https://github.com/urmzd/embed-md/commit/57857ce19afe77410685cd0dbc4ae064c614c439))

### Documentation

- regenerate CHANGELOG.md from tag history ([8eb5480](https://github.com/urmzd/embed-md/commit/8eb5480bf6c3f8f68994578c06b7a221d0efde02))
- update action definition, README, and examples for Rust rewrite ([4555cdc](https://github.com/urmzd/embed-md/commit/4555cdcde4cfdda1ccd530a471e07c39aa88394d))

### Miscellaneous

- parallelize fmt/lint/test checks and remove redundant release build ([1135edb](https://github.com/urmzd/embed-md/commit/1135edb3acb710e67c7ea9e3796186ea53334592))
- update CI/CD and config for Rust + semantic-release ([7087c15](https://github.com/urmzd/embed-md/commit/7087c158b1065592b31a686879e0786289941b5e))
- remove Node.js artifacts and Docker-based action ([5ff94f9](https://github.com/urmzd/embed-md/commit/5ff94f9cfe54990bf2de97618782774434a9cf09))

### Contributors

- @urmzd


## 1.3.4 (2024-01-17)

### Bug Fixes

- ensure default VERSION always exists ([51e6255](https://github.com/urmzd/embed-md/commit/51e6255164c6a952110160eb08316a92d9aa9ccc))

### Documentation

- update README.md ([3d0eb33](https://github.com/urmzd/embed-md/commit/3d0eb3354d175276d5430b3fc16566cda3e38e34))
- describe README.md ([084e110](https://github.com/urmzd/embed-md/commit/084e110e2e313d181ceb1a307008d3521f175bff))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.3.3...v1.3.4)

## 1.3.3 (2024-01-13)

### Bug Fixes

- git commit-message ([7f84f22](https://github.com/urmzd/embed-md/commit/7f84f223008462aa1fc72d8a17408db124739607))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.3.2...v1.3.3)

## 1.3.2 (2024-01-13)

### Bug Fixes

- update references before pushing up ([4d57617](https://github.com/urmzd/embed-md/commit/4d576172f2ecbcf9db86ca33c6d28129d2c2764a))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.3.1...v1.3.2)

## 1.3.1 (2024-01-13)

### Bug Fixes

- add final git push ([b0d2459](https://github.com/urmzd/embed-md/commit/b0d2459001b5ac3be0ec05c0854430de0d40241e))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.3.0...v1.3.1)

## 1.3.0 (2024-01-13)

### Features

- dry run ([a9582bb](https://github.com/urmzd/embed-md/commit/a9582bb946353183e527dd707d1542752982cd70))

### Bug Fixes

- **entry**: ensure staged diff ([81feb06](https://github.com/urmzd/embed-md/commit/81feb069fa9411f5fc08e34405e6a465296b560c))
- syntax ([d9d4e02](https://github.com/urmzd/embed-md/commit/d9d4e0203e0f3f3cee144ea3da3d46b7319a2325))
- examples ([2cee528](https://github.com/urmzd/embed-md/commit/2cee528e4b12f87fd847c6d2831b2a3359f1f3af))
- permission errors ([0baef5e](https://github.com/urmzd/embed-md/commit/0baef5e0642e9829a146fab3f3c2a7731d7897c4))
- ensure /github is considered safe ([1156862](https://github.com/urmzd/embed-md/commit/115686219bc0eaae9ef5e63a2335dd5387246fb3))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.2.3...v1.3.0)

## 1.2.3 (2024-01-12)

### Bug Fixes

- ensure latest version is pushed up ([b9100de](https://github.com/urmzd/embed-md/commit/b9100def95c548b91311d36d45ee2cd23ff9c1b4))
- use self reference ([fb0bab4](https://github.com/urmzd/embed-md/commit/fb0bab412afd5db5cee94fd931989bf9641d8643))

### Miscellaneous

- update version to 1.2.3 ([485a047](https://github.com/urmzd/embed-md/commit/485a047a42a0ac269ae237d66a3aeee47cdd74d7))

### Contributors

- @urmzd
- @github-actions[bot]

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.2.2...v1.2.3)

## 1.2.2 (2024-01-12)

### Bug Fixes

- continue on error for npm version ([86ab56f](https://github.com/urmzd/embed-md/commit/86ab56f997a5fa9e4e162676fac5b79a113a552a))
- workdir ([6aba7e6](https://github.com/urmzd/embed-md/commit/6aba7e6d06bb228fc906af23d8f36451ebe46a99))
- script pathing ([7304f39](https://github.com/urmzd/embed-md/commit/7304f395bca78be27bea1a530e588d992eb7f904))
- **ci**: path references ([325b94f](https://github.com/urmzd/embed-md/commit/325b94f83786c95a3852c1cb7a7cfb266113208b))
- **ci**: ensure correct version is built ([019b124](https://github.com/urmzd/embed-md/commit/019b1244e87923748e6b3dfe7a723f97b237148d))
- ci workflow ([b26b4a0](https://github.com/urmzd/embed-md/commit/b26b4a0c14a025cfdd09272eddff15b8f9138e09))
- avoid permission issues ([89c3037](https://github.com/urmzd/embed-md/commit/89c30375115971e16b589b82a9b88e854db0cd28))
- ensure dependencies are installed ([7f89173](https://github.com/urmzd/embed-md/commit/7f89173a208f7ea26765526d3146ebb659a9f52a))
- update_version script ([e4241f6](https://github.com/urmzd/embed-md/commit/e4241f683a008f97a15301bb52d204ddecd4a11f))

### Miscellaneous

- run self ([6d67cd4](https://github.com/urmzd/embed-md/commit/6d67cd446db8e53f7edf347db08dd4008885cee6))
- update version to 1.2.2 ([e51871f](https://github.com/urmzd/embed-md/commit/e51871f9071427fb61b4a791e2ecc7f76937c5ee))
- fix condition ([8992d11](https://github.com/urmzd/embed-md/commit/8992d1194bf731cba6be9b05ca82326a128f3afa))
- prepare release ([c64ffed](https://github.com/urmzd/embed-md/commit/c64ffede96009a7a283e932f28b09bea5c697040))
- ensure package.json is updated ([94d3b35](https://github.com/urmzd/embed-md/commit/94d3b357501af8b98a0a5ae050d45058ad673ab7))

### Contributors

- @urmzd
- @github-actions[bot]

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.2.1...v1.2.2)

## 1.2.1 (2024-01-09)

### Bug Fixes

- **release**: define setps for ci.yml ([49bbd83](https://github.com/urmzd/embed-md/commit/49bbd832a6c84e0e7a332ac71e7a813da4b31b2d))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.2.0...v1.2.1)

## 1.2.0 (2024-01-08)

### Features

- ensure changes are pushed ([a71289f](https://github.com/urmzd/embed-md/commit/a71289f2fa674581af50ec0792b26346ebd65bc4))

### Bug Fixes

- ignore commit ([1d5331b](https://github.com/urmzd/embed-md/commit/1d5331bc58b358004c094e66b7b967a501f5b910))
- ensure packag is updated ([9be658a](https://github.com/urmzd/embed-md/commit/9be658a3e6922a1c69fc8c99acfbf24b7a9c6556))
- ci add angular support ([0717315](https://github.com/urmzd/embed-md/commit/07173156ed3ac48f9d65eec5e76f3d94cdff1394))
- releaserc ([cb6c4db](https://github.com/urmzd/embed-md/commit/cb6c4db2ae3154ad8e74fd99ff1a936a5523ca19))
- ensure version is updated ([1182978](https://github.com/urmzd/embed-md/commit/1182978fa086496d85a4ea3b30225bb0a16414c5))

### Refactoring

- split steps ([42685ce](https://github.com/urmzd/embed-md/commit/42685ce0ae7901a9967687c67b25c6642ed4e5be))

### Miscellaneous

- commit only when changes occur ([df1a2f4](https://github.com/urmzd/embed-md/commit/df1a2f4fa88c132a1eb4ef5d0629acd72c01c77b))
- push version changes ([2cb4175](https://github.com/urmzd/embed-md/commit/2cb41759841d01eb6b97d7b5882d98ed03cfcb71))
- continue ([2ff169d](https://github.com/urmzd/embed-md/commit/2ff169da2147c9b9ed46d6b0d3687e0c76bd0dd1))
- ensure package is not released ([afe38de](https://github.com/urmzd/embed-md/commit/afe38deefb86c7d4467dc1dd630dd0d22432bd58))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.1.0...v1.2.0)

## 1.1.0 (2024-01-08)

### Features

- add release notes generator ([545b968](https://github.com/urmzd/embed-md/commit/545b968614005355c5d2f113362333db88a0256e))

### Bug Fixes

- assertion ([a35981e](https://github.com/urmzd/embed-md/commit/a35981ef2044ac7b262833daae6a95ecfdb60f27))
- add assertion for modified example.md file ([8d89f3a](https://github.com/urmzd/embed-md/commit/8d89f3a84c67b9cb1ee1dca81a3ecb07aa9a1c45))
- remove example update ([50fcff8](https://github.com/urmzd/embed-md/commit/50fcff8fead429be4f7e3c307db6e0beb11ad2eb))
- add working directory as a safe dir ([851fba6](https://github.com/urmzd/embed-md/commit/851fba62cc9b81c8bbb89e6a1bd30d6a2ce61e5b))
- example run ([8480812](https://github.com/urmzd/embed-md/commit/84808124ba7fe4d295eb432e3d991a95247588c5))
- add write permissions for contents and pull requests ([200b688](https://github.com/urmzd/embed-md/commit/200b6886489a65723cc59075a2d9f5c7cd8bfd96))
- update Dockerfile and entrypoint.sh ([1cd9cff](https://github.com/urmzd/embed-md/commit/1cd9cff512841fb863842c055754d55592bb191a))
- entrypoint command ([58b826e](https://github.com/urmzd/embed-md/commit/58b826e0f6fa9d305bdb1412d6ecd40c9b61bdad))
- ensure CD pipeline only runs on merge to main ([8981a23](https://github.com/urmzd/embed-md/commit/8981a232f7303a339ccc836b0e3513df99ec7cef))
- ensure semantic-release works ([6164371](https://github.com/urmzd/embed-md/commit/61643711cbfbc66707958b0f20db9c7792e123d2))

### Refactoring

- reduce load up logic ([d67342a](https://github.com/urmzd/embed-md/commit/d67342ab2359ae5a9df08e8560bf2687db0606c2))
- entrypoint.sh script ([a06d904](https://github.com/urmzd/embed-md/commit/a06d90401ed5c183858228fc405c06e2642146dc))
- Docker run command and update default commit author ([17de280](https://github.com/urmzd/embed-md/commit/17de2809ce40255cd9f0040dcd201a47d4fb4ea1))
- split action into steps ([252196e](https://github.com/urmzd/embed-md/commit/252196e8100b339a5ebca3805d776068b85d0362))

### Miscellaneous

- ensure image is pushed up ([199dd2c](https://github.com/urmzd/embed-md/commit/199dd2c3490463e829b491973fa536633df90bcd))
- assertion on changes ([1a7b0d1](https://github.com/urmzd/embed-md/commit/1a7b0d10e0524900e630c8fad281521e6658c1a4))
- add step to assert changes ([199d1fb](https://github.com/urmzd/embed-md/commit/199d1fbe1e1047b1b2363fdce36dc31d9ee77384))
- ensure version is escaped correctly ([2b02a56](https://github.com/urmzd/embed-md/commit/2b02a56f94f7eecb0eb5635cfae6cf308017ede6))
- remove reference to invalid file ([71d520c](https://github.com/urmzd/embed-md/commit/71d520c7cd07c3cdf912457f6c846de8aebe68e0))
- ensure VERSION is available ([d5a0a4e](https://github.com/urmzd/embed-md/commit/d5a0a4eae6bdeaeb1c2d5929d47fcb8fc3e4d9c2))
- ensure environment variables are set ([467da41](https://github.com/urmzd/embed-md/commit/467da415c28751109b1c11e76f2ae51c754fc7b2))
- ensure semantic-release is updated ([cfc54ce](https://github.com/urmzd/embed-md/commit/cfc54cef3bbe51e65d05d551154ca9d3b05a295f))
- ensure pipeline executes on push ([3c9b9b8](https://github.com/urmzd/embed-md/commit/3c9b9b8ced22d8fdc4aab059b269a13e22ad2f21))
- random ([75541f7](https://github.com/urmzd/embed-md/commit/75541f7e1934417193c3b15a1f26774fc4a89779))
- using public action ([f33939f](https://github.com/urmzd/embed-md/commit/f33939fbd5fcd382a663507b72961c5effaaef28))

### Contributors

- @urmzd
- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.0.2...v1.1.0)

## 1.0.2 (2023-04-19)

### Bug Fixes

- remove early fail ([65b58bb](https://github.com/urmzd/embed-md/commit/65b58bbe629fe3fc3fe88f23d9598933545bee5b))
- add workspace first ([447aa27](https://github.com/urmzd/embed-md/commit/447aa272258beb6507ed573cd400f8d0af9b95fb))
- fail early ([900c64e](https://github.com/urmzd/embed-md/commit/900c64e26e54cfebef674e113880ede999c82996))
- config ([8b75dd4](https://github.com/urmzd/embed-md/commit/8b75dd4abdccd7d3dcbc70f9542609f985871500))
- add directory globally ([93c4b50](https://github.com/urmzd/embed-md/commit/93c4b50b0d67ca02ee38c1788d963603eb7aefb4))
- push from workspace ([6e66804](https://github.com/urmzd/embed-md/commit/6e668044f667b6539687b8396455bf512defd5b0))
- cd to rootlevel ([16a71fa](https://github.com/urmzd/embed-md/commit/16a71fadba281efb00fcaf942c9c648e46d91c1d))
- ensure github workspace can be pushed from ([83a5833](https://github.com/urmzd/embed-md/commit/83a583301bc066fb85327a7701cd1e69f77cc22b))
- always push to current branch ([1e42c18](https://github.com/urmzd/embed-md/commit/1e42c18419da37ab09c9da8f31b560bfc1464a24))
- ensure current directory is considered safe ([ff405c3](https://github.com/urmzd/embed-md/commit/ff405c33b0c50e6e0d8a0ef98a482360ae22b369))
- ensure docker entrypoint exists ([4115955](https://github.com/urmzd/embed-md/commit/411595530a666796022d4c3b9eb2a1df198d9d72))

### Documentation

- credit embedme ([14ccfff](https://github.com/urmzd/embed-md/commit/14ccfff7c181772efb8544a4e6b8150adbec119f))

### Miscellaneous

- trigger on src mods ([c50d09b](https://github.com/urmzd/embed-md/commit/c50d09b778a69c8e37225d83987d195003b5a8b5))
- run on PR ([677f558](https://github.com/urmzd/embed-md/commit/677f558e22774a280b267dc506d6359747f7287e))
- embed example using self ([735f598](https://github.com/urmzd/embed-md/commit/735f5981ebef6390212c51cc14f569a96a76d789))
- run on pipeline changes ([5381d1c](https://github.com/urmzd/embed-md/commit/5381d1ccd5ff217de20b10fcb3b30eefe6e7a100))
- run test on every push ([ef53fb7](https://github.com/urmzd/embed-md/commit/ef53fb75478a9c1ef5e2f08c8b8083dcae01c5b6))
- prevent multiple jobs from starting ([7315c3d](https://github.com/urmzd/embed-md/commit/7315c3d163f7765f7957f6a85c82da02447eb633))

### Contributors

- @urmzd
- embed-md bot

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.0.1...v1.0.2)

## 1.0.1 (2022-12-08)

### Miscellaneous

- change example ([fe9d07a](https://github.com/urmzd/embed-md/commit/fe9d07aa2e58b195afe84c63c184e0825ebcc81e))

### Contributors

- @urmzd

[Full Changelog](https://github.com/urmzd/embed-md/compare/v1.0.0...v1.0.1)

## 1.0.0 (2022-12-05)

### Bug Fixes

- add branding ([eb5cd51](https://github.com/urmzd/embed-md/commit/eb5cd511b59ddc09772d370613e58922d32e892e))
- run workflow only on release ([7ff55c8](https://github.com/urmzd/embed-md/commit/7ff55c8d5e4c904b2ba4f3464b888b770648df75))
- ensure changes are pushed ([cc39f97](https://github.com/urmzd/embed-md/commit/cc39f9780927ae3eca15d2a43dd166a4688161b1))
- ensure repo is checked-out ([9e240d1](https://github.com/urmzd/embed-md/commit/9e240d1980c4bbce983785105d0d8cdeb63d076f))

### Documentation

- describe how to use action ([9c020d9](https://github.com/urmzd/embed-md/commit/9c020d931d61d97bf15f38a1d0bb8dfb152e1a1b))

### Miscellaneous

- embed example using self ([e0c600d](https://github.com/urmzd/embed-md/commit/e0c600dd44c8a0fea082a54cccc2e413ede56ba8))
- update name ([9c5aeca](https://github.com/urmzd/embed-md/commit/9c5aecab20a69ede4c740754ca25c5c9767d6815))
- self action ([2e75faf](https://github.com/urmzd/embed-md/commit/2e75faf069d229aea1edcd4a48fcbf19fefa7620))
- create release file ([cbe435e](https://github.com/urmzd/embed-md/commit/cbe435ef7fc6197768095d316801a746f5329ee0))
- update name ([7a515a4](https://github.com/urmzd/embed-md/commit/7a515a45613e55b7866e28446c666f42211c3a8c))
- update name ([9d83328](https://github.com/urmzd/embed-md/commit/9d83328adaccf978711d15786dbc43bd06e3818d))
- create metadata file ([718841b](https://github.com/urmzd/embed-md/commit/718841b3b000e8fe7c9cfd5aec6c5e17aaa082c3))

### Contributors

- @urmzd
- embed-md bot
