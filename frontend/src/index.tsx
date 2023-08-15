import './styles/app.css'
import 'vite/modulepreload-polyfill'
import { createRoot } from 'react-dom/client'
import { createInertiaApp } from '@inertiajs/react'
import { InertiaProgress } from '@inertiajs/progress'

InertiaProgress.init()

createInertiaApp({
	title: title => (title ? `${title} - Ping CRM` : 'Ping CRM'),
	resolve: name => import.meta.glob('./Pages/**/*.tsx', { eager: true })[`./Pages/${name}.tsx`],
	setup({ el, App, props }) {
		createRoot(el).render(<App {...props} />)
	},
})
