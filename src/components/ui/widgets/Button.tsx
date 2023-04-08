import styles from "@/styles/ui/widgets/Button.module.scss";
import classNames from "classnames";
import React from "react";

type ButtonProps = {
  text: string,
  onClick?: () => void,
  style?: ButtonStyles,
  children?: React.ReactNode,
}

// eslint-disable-next-line no-shadow
export const enum ButtonStyles {
  DEFAULT,
  GREEN,
  RED,
  DISABLED,
}

export default function Button({ text, onClick = () => null, style = ButtonStyles.DEFAULT, children }: ButtonProps) {

  // eslint-disable-next-line prefer-const
  let variantedStyle: string = "";
  switch (style) {
    default:
    case ButtonStyles.DEFAULT:
      variantedStyle = "";
      break;
    case ButtonStyles.GREEN:
      variantedStyle = styles.__green;
      break;
    case ButtonStyles.RED:
      variantedStyle = styles.__red;
      break;
    case ButtonStyles.DISABLED:
      variantedStyle = styles.__disabled;
      break;
  }


  return <>
    <div className={classNames(styles.button, variantedStyle)} onClick={onClick}>
      {children}{text}
    </div>
  </>;
}
