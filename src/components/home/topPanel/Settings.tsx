import Button, { ButtonStyles } from "@/components/ui/widgets/Button";
import styles from "@/styles/home/topPanel/Settings.module.scss";
import Image from "next/image";

export default function Settings() {


  return <Button text="" style={ButtonStyles.CLEAR}>
    <Image className={styles.icon} src="/settings.svg" alt="settings" width={24} height={24}/>
  </Button>;
}