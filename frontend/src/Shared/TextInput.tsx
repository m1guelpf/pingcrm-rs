import { FC, HTMLAttributes, useId } from 'react'

type Props = {
	label: string
	type?: string
	error?: string
	value: string
	onChange: (value: string) => void
} & Omit<HTMLAttributes<HTMLInputElement>, 'onChange' | 'value'>

const TextInput: FC<Props> = ({ label, error, value, onChange, className, type = 'text', ...props }) => {
	const id = useId()

	return (
		<div className={className}>
			{label && (
				<label className="form-label" htmlFor={id}>
					{label}:
				</label>
			)}
			<input
				id={id}
				type={type}
				value={value}
				onChange={e => onChange(e.target.value)}
				className={`form-input ${error ? 'error' : ''}`}
				{...props}
			/>
			{error && <div className="form-error">{error}</div>}
		</div>
	)
}

export default TextInput
