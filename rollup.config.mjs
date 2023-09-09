import {terser} from "rollup-plugin-terser";
import typescript from '@rollup/plugin-typescript';
import nodeResolve from "@rollup/plugin-node-resolve";
import json from '@rollup/plugin-json';
import commonjs from '@rollup/plugin-commonjs';

const license = "/**!\n" +
    " * cazan-cli  v0.1.0 (https://github.com/AeliaDev/cazan-cli)\n" +
    " * Copyright 2023 The Cazan Authors\n" +
    " * Licensed under MIT (https://github.com/AeliaDev/cazan-cli/blob/main/LICENSE)\n" +
    " */"

export default {
    input: 'src/main.ts',
    output: [
        {
            file: 'dist/cazan-cli.js',
            format: 'cjs',
            banner: license,
        },{
            file: 'dist/cazan-cli.min.js',
            format: 'cjs',
            name: 'cazan',
            banner: license,
            plugins: [terser()]
        }
    ],
    plugins: [typescript(), json(), nodeResolve(), commonjs()],
};
