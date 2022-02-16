/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-12 10:01:10
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-14 11:56:48
 * @FilePath: /layer-painter/wasm/tailwind.config.js
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */
module.exports = {
  content: [
    "./input.css",
    "./src/**/*.rs",
    "./src/*.rs",
    "./index.html"
  ],
  theme: {
    extend: {
      spacing: {
        '128': '32rem',
      }
    },
  },
  plugins: [],
}
