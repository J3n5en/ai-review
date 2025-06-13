#!/usr/bin/env node

import fs from 'fs'
import path from 'path'

// 读取版本配置
const versionConfig = JSON.parse(fs.readFileSync('version.json', 'utf8'))
const { version, name, description } = versionConfig

console.log(`更新版本到 ${version}...`)

// 更新 package.json
const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'))
packageJson.version = version
fs.writeFileSync('package.json', JSON.stringify(packageJson, null, 2) + '\n')
console.log('✅ 更新 package.json')

// 更新 Cargo.toml
let cargoToml = fs.readFileSync('Cargo.toml', 'utf8')
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${version}"`)
fs.writeFileSync('Cargo.toml', cargoToml)
console.log('✅ 更新 Cargo.toml')

// 更新 tauri.conf.json
const tauriConf = JSON.parse(fs.readFileSync('tauri.conf.json', 'utf8'))
tauriConf.version = version
tauriConf.productName = name
tauriConf.bundle.shortDescription = `${name} - ${description.split('，')[0]}`
tauriConf.bundle.longDescription = description
fs.writeFileSync('tauri.conf.json', JSON.stringify(tauriConf, null, 2) + '\n')
console.log('✅ 更新 tauri.conf.json')

// 更新 index.html
let indexHtml = fs.readFileSync('index.html', 'utf8')
indexHtml = indexHtml.replace(/<title>.*<\/title>/, `<title>${name}</title>`)
fs.writeFileSync('index.html', indexHtml)
console.log('✅ 更新 index.html')

console.log(`🎉 版本更新完成: ${version}`)
