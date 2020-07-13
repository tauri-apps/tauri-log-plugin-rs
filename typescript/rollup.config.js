import {
  terser
} from 'rollup-plugin-terser'
import resolve from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import babel from '@rollup/plugin-babel'
import typescript from '@rollup/plugin-typescript'
import pkg from './package.json'

export default [{
    input: {
      'index': './src/index.ts'
    },
    treeshake: true,
    perf: true,
    output: [{
        dir: 'dist/',
        entryFileNames: '[name].js',
        format: 'cjs',
        exports: 'named',
        globals: {}
      },
      {
        dir: 'dist/',
        entryFileNames: '[name].mjs',
        format: 'esm',
        exports: 'named',
        globals: {}
      }
    ],
    plugins: [
      commonjs({}),
      resolve({
        // pass custom options to the resolve plugin
        customResolveOptions: {
          moduleDirectory: 'node_modules'
        }
      }),
      typescript({
        declaration: true,
        declarationDir: 'dist',
        rootDir: 'src'
      }),
      terser()
    ],
    external: [
      ...Object.keys(pkg.dependencies || {}),
      ...Object.keys(pkg.peerDependencies || {})
    ],
    watch: {
      chokidar: true,
      include: 'src/**',
      exclude: 'node_modules/**'
    }
  },
  {
    input: {
      'bundle': './src/index.ts'
    },
    output: [{
      name: '__TAURI_PLUGIN_LOG__',
      dir: 'dist/',
      entryFileNames: 'plugin.umd.js',
      format: 'umd',
      globals: {}
    }],
    plugins: [
      typescript(),
      babel(),
      terser(),
      resolve({
        // pass custom options to the resolve plugin
        customResolveOptions: {
          moduleDirectory: 'node_modules'
        }
      })
    ],
    external: [
      ...Object.keys(pkg.dependencies || {}),
      ...Object.keys(pkg.peerDependencies || {})
    ]
  }
]