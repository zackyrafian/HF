type Props = {
  name: string;
  type?: string;
  placeholder?: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
};

export default function FormField({
  name,
  type = "text",
  placeholder,
  value,
  onChange,
}: Props) {
  return (
    <input
      name={name}
      type={type}
      placeholder={placeholder}
      value={value}
      onChange={onChange}
      className="w-full p-2 border rounded"
      required
    />
  );
}
