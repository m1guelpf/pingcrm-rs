import { ButtonHTMLAttributes, FC, PropsWithChildren } from 'react'

const LoadingButton: FC<PropsWithChildren<{ loading: boolean } & ButtonHTMLAttributes<HTMLButtonElement>>> = ({
	children,
	loading,
	...props
}) => {
	return (
		<button disabled={loading} className="flex items-center" {...props}>
			{loading && <div className="btn-spinner mr-2" />}
			{children}
		</button>
	)
}

export default LoadingButton
