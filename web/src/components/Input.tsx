
interface InputProps {
  label: string;
  type: React.HTMLInputTypeAttribute;
  placeholder: string | undefined;
  value: string | number | readonly string[] | undefined;
  setValue: (value: string) => void;
}

export const Input = ({ label, type, placeholder, value, setValue }: InputProps) => {
  return (
    <div>
      <p>{label}</p> 
      <input 
        type={type}
        placeholder={placeholder}
        value={value || ""}
        onChange={(e) => setValue(e.target.value)}
        
        className="bg-slate-600 rounded-md active:bg-slate-700 p-1 px-2 w-full"
      />
    </div>
  )
}
