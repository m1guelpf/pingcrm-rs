import fs from 'fs'
import { AddressInfo } from 'net'
import { Plugin, defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

export default defineConfig(() => ({
	plugins: [react(), axum()],
	clearScreen: false,
	resolve: {
		alias: [{ find: '@', replacement: '/src' }],
	},
	build: {
		manifest: true,
		rollupOptions: {
			input: { 'src/index.tsx': './src/index.tsx' },
		},
	},
}))

let exitHandlersBound = false
const axum = (): Plugin => ({
	name: 'axum',
	configureServer: server => {
		server.httpServer?.once('listening', () => {
			const address = server.httpServer?.address()

			if (isAddressInfo(address)) {
				const viteDevServerUrl = `http://${
					address.family === 'IPv6' ? `[${address.address}]` : address.address
				}:${address.port}`
				fs.writeFileSync('./dist/.vite-dev', viteDevServerUrl)
			}
		})

		if (!exitHandlersBound) {
			const clean = () => {
				if (fs.existsSync('./dist/.vite-dev')) {
					fs.rmSync('./dist/.vite-dev')
				}
			}

			process.on('exit', clean)
			process.on('SIGINT', process.exit)
			process.on('SIGTERM', process.exit)
			process.on('SIGHUP', process.exit)

			exitHandlersBound = true
		}
	},
})

const isAddressInfo = (x: string | AddressInfo | null | undefined): x is AddressInfo => typeof x === 'object'
