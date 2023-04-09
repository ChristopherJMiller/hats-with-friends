
interface ButtonProps {
  text: string;
  disabled: boolean;
  onClick: () => void;
}

export const Button = ({ text, disabled, onClick }: ButtonProps) => {
  return (
    <div className={`${disabled ? "italic" : "hover:cursor-pointer hover:bg-teal-600 active:bg-teal-800"} bg-teal-500 rounded-md text-center px-3 p-2`} onClick={() => !disabled ? onClick() : null}>{text}</div>
  )
};
