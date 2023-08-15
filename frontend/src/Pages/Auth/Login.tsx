import Logo from '@/Shared/Logo'
import TextInput from '@/Shared/TextInput'
import { FormEvent, useCallback } from 'react'
import { Head, useForm } from '@inertiajs/react'
import LoadingButton from '@/Shared/LoadingButton'

const Login = () => {
	const { data, setData, post, processing, errors } = useForm({
		email: 'johndoe@example.com',
		password: 'secret',
		remember: false,
	})

	const login = useCallback(
		(event: FormEvent<HTMLFormElement>) => {
			event.preventDefault()

			post('/login')
		},
		[post]
	)

	return (
		<>
			<Head title="Login" />
			<div className="flex items-center justify-center p-6 min-h-screen bg-indigo-800">
				<div className="w-full max-w-md">
					<Logo className="block mx-auto w-full max-w-xs fill-white" height="50" />
					<form onSubmit={login} className="mt-8 bg-white rounded-lg shadow-xl overflow-hidden">
						<div className="px-10 py-12">
							<h1 className="text-center text-3xl font-bold">Welcome Back!</h1>
							<div className="mt-6 mx-auto w-24 border-b-2" />
							<TextInput
								value={data.email}
								onChange={email => setData('email', email)}
								error={errors.email}
								className="mt-10"
								label="Email"
								type="email"
								autoFocus
								autoCapitalize="off"
							/>
							<TextInput
								value={data.password}
								onChange={password => setData('password', password)}
								error={errors.password}
								className="mt-6"
								label="Password"
								type="password"
							/>
							<label className="flex items-center mt-6 select-none" htmlFor="remember">
								<input
									id="remember"
									type="checkbox"
									className="mr-1"
									checked={data.remember}
									onChange={e => setData('remember', e.target.checked)}
								/>
								<span className="text-sm">Remember Me</span>
							</label>
						</div>
						<div className="flex px-10 py-4 bg-gray-100 border-t border-gray-100">
							<LoadingButton loading={processing} className="btn-indigo ml-auto" type="submit">
								Login
							</LoadingButton>
						</div>
					</form>
				</div>
			</div>
		</>
	)
}

export default Login
