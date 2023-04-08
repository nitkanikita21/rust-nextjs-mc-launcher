import classNames from "classnames";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

import Button, { ButtonStyles } from "../../ui/widgets/Button";
import styles from "@/styles/home/login/Login.module.scss";
import DialogContainer from "@/components/ui/DialogContainer";

export default function Login() {
  const [logined, setLogined] = useState(false);
  const [nickname, setNickname] = useState<string>("");
  const [skinUrl, setSkinUrl] = useState<string>("");

  const [isDialogActive, setDialogActive] = useState<boolean>(false);


  function updateLoginStatus() {
    invoke<boolean>("is_logined")
      .then(n => setLogined(n));
  }
  function updateNickname() {
    invoke<string>("get_username")
      .then(n => setNickname(n));
  }
  function updateSkinUrl() {
    invoke<string>("get_user_head_render_url")
      .then(n => setSkinUrl(n));
  }

  function login() {
    invoke<string>("login");
  }
  function unlogin() {
    invoke<string>("unlogin");
  }

  useEffect(() => {
    updateLoginStatus();
    updateNickname();
    updateSkinUrl();

    listen("login_status", () => {
      updateLoginStatus();
      updateNickname();
      updateSkinUrl();
      console.log("login_status");
    });
  });

  function openAcceptUnlogin() {
    setDialogActive(true);
  }
  function acceptUnlogin() {
    setDialogActive(false);
    unlogin();
  }
  function cancelUnlogin() {
    setDialogActive(false);
  }

  let loginBtn = <></>;
  if (!logined) {
    loginBtn = <Button text={"Увійти"} onClick={login} style={ButtonStyles.GREEN} />;

  } else {
    loginBtn = <Button text={nickname} onClick={openAcceptUnlogin} style={ButtonStyles.DEFAULT}>
      {/* // eslint-disable-next-line @next/next/no-img-element */}
      <img src={skinUrl} alt="skin_url" className={styles.head_skin} />
    </Button>;
  }

  return <>
    <DialogContainer active={isDialogActive} title="Ви хочете вийти з аккаунту">
      <p className={styles.dialog_text}>Якщо ви це зробите, вам потрібно буде знову авторизовуватися в свій аккаунт</p>
      <div className={styles.dialog_btns}>
        <Button text="Вийти" onClick={acceptUnlogin} style={ButtonStyles.RED} />
        <Button text="Назад" onClick={cancelUnlogin} style={ButtonStyles.DEFAULT} />
      </div>
    </DialogContainer>
    {loginBtn}
  </>;
}