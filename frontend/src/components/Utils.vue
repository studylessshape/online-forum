<script>
import Cookies from 'js-cookie';

/**
 * Check is valid format of email
 * @param {String} str 
 */
function isEmail(str) {
  if (str == null || str == undefined || str.length < 4)
    return false;
  var at_pos = str.indexOf("@");
  var dot_pos = str.lastIndexOf(".");
  // check is email style
  return !(at_pos < 1 || dot_pos < at_pos || dot_pos + 2 >= str.length);
}

/**
 * 
 * @param {String} password 
 */
function checkPasswordValid(password) {
  if (password.length < 8) {
    return false;
  }
  var count = 0;
  var low_word_re = new RegExp("[a-z]");
  var big_word_re = new RegExp("[A-Z]");
  var number_re = new RegExp("[0-9]");
  var special_re = new RegExp("[~`!@#$%^&*(_)+.=?,;:'\"/]");
  if (low_word_re.exec(password)) {
    count++;
  }
  if (big_word_re.exec(password)) {
    count++;
  }
  if (number_re.exec(password)) {
    count++;
  }
  if (special_re.exec(password)) {
    count++;
  }
  if (count < 3) {
    return false;
  }
  return true;
}

/**
 * Format to display string
 * @param {String} time 
 */
function displaySqlDatetime(time) {
  if (!time || time == undefined)
    return time;
  var arr = time.split('T');
  var ymd = arr[0].split('-');
  return ymd[0] + "年" + ymd[1] + "月" + ymd[2] + "日 " + arr[1];
}

/**
 * 
 * @param {Date} date 
 */
function displayDate(date) {
  if (date) {
    var hours = date.getHours() > 10 ? date.getHours() : `0${date.getHours()}`;
    var minutes = date.getMinutes() > 10 ? date.getMinutes() : `0${date.getMinutes()}`;
    var seconds = date.getSeconds() > 10 ? date.getSeconds() : `0${date.getSeconds()}`;
    return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日 ${hours}:${minutes}:${seconds}`;
  }
  return '';
}
/**
 * 
 * @param {String} id 
 * @param {boolean} is_valid 
 */
function setValid(id, is_valid) {
  var id = "#" + id;
  if (is_valid) {
    $(id).removeClass("is-invalid");
    $(id).addClass("is-valid");
  } else {
    $(id).removeClass("is-valid");
    $(id).addClass("is-invalid");
  }
}

/**
 * 
 * @param {String} id 
 */
function removeValid(id) {
  var id = "#" + id;
  $(id).removeClass("is-invalid");
  $(id).removeClass("is-valid");
}

const imageFileTypes = [
  "image/bmp",
  "image/gif",
  "image/jpeg",
  "image/png",
  "image/svg+xml",
  "image/tiff",
  "image/webp",
];

/**
 * 
 * @param {File} file 
 * @param {String[]} fileTypes 
 */
function validFileType(file, fileTypes) {
  return fileTypes.includes(file.type);
}

/**
 * 
 * This function don't simplify anything.
 * 
 * Inside is this: 
 * ```js
 * return await fetch(url, {
 *   method: "POST",
 *   mode: "cors",
 *   body: formFile,
 *   cache: "no-store",
 * })
 * ```
 * @param {FormData} formFile 
 * @param {String} url 
 */
async function uploadFile(formFile, url) {
  return await fetch(url, {
    method: "POST",
    mode: "cors",
    body: formFile,
    cache: "no-store",
  })
}

class CookieWatch {
  constructor() {
    this.postEventName = {
      onSetCookie: "on-set-cookie",
      onRemoveCookie: "on-remove-cookie",
    };
    this.postListener = new EventTarget();
    this.postListener.addEventListener(this.postEventName.onSetCookie, (cob) => {
      Cookies.set(cob.detail.cookieName, cob.detail.newValue, { path: '/', expires: 30 });
    });
    this.postListener.addEventListener(this.postEventName.onRemoveCookie, (cName) => {
      Cookies.remove(cName.detail.cookieName);
    })
  }

  /**
   * The `func` input params is like this:
   * ```js
   * // for type `'this.postEventName.onRemoveCookie'`
   * cookieWatch.addListener(this.postEventName.onRemoveCookie, (cob)=>{
   *   console.log(`cookie-name: ${cob.detail.cookieName};`);
   * })
   * 
   * // for type `'this.postEventName.onSetCookie'`
   * cookieWatch.addListener(this.postEventName.onSetCookie, (cob)=>{
   *   console.log(`cookie-name: ${cob.detail.cookieName}; new-value: ${cob.detail.newValue}`);
   * })
   * ```
   * 
   * @param {EventListenerOrEventListenerObject} func 
   */
  addListener(type, func) {
    this.postListener.addEventListener(type, func);
  }

  /**
   * 
   * @param {EventListenerOrEventListenerObject} func 
   */
  removeListener(type, func) {
    this.postListener.removeEventListener(type, func);
  }

  /**
   * 
   * @param {String} cookieName 
   * @param {*} newValue 
   */
  setCookie(i_cookieName, i_newValue) {
    var setEvent = new CustomEvent(this.postEventName.onSetCookie, {
      detail: {
        cookieName: i_cookieName,
        newValue: i_newValue,
      }
    });
    return this.postListener.dispatchEvent(setEvent);
  }

  /**
   * 
   * @param {String} i_cookieName 
   */
  removeCookie(i_cookieName) {
    var removeEvent = new CustomEvent(this.postEventName.onRemoveCookie, {
      detail: {
        cookieName: i_cookieName,
      }
    });
    return this.postListener.dispatchEvent(removeEvent);
  }

  /**
   * 
   * @param {String} cookieName 
   */
  getCookie(cookieName) {
    return Cookies.get(cookieName);
  }
}

/**
 * Use this object to modify cookie, it will callback event that you add
 */
const cookieWatch = new CookieWatch();

export {
  isEmail,
  checkPasswordValid,
  displaySqlDatetime,
  displayDate,
  setValid,
  removeValid,
  imageFileTypes,
  validFileType,
  uploadFile,
  cookieWatch,
}
</script>