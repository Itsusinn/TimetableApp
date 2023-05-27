package cat.itsusinn.timetable

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.Crossfade
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.sp
import cat.itsusinn.timetable.ffi.initLogger
import cat.itsusinn.timetable.ui.about.AboutScreen
import cat.itsusinn.timetable.ui.home.HomeScreen
import com.mikepenz.iconics.compose.Image
import com.mikepenz.iconics.typeface.library.fontawesome.FontAwesome

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        initLogger()
        super.onCreate(savedInstanceState)
        setContent {
            MainAppContent()
        }
    }
}

@Composable
fun MainAppContent() {
    val homeScreenState = rememberSaveable { mutableStateOf(BottomNavType.HOME) }
    val coroutineScope = rememberCoroutineScope()
    Column {
        HomeScreenContent(
            homeScreen = homeScreenState.value,
            modifier = Modifier.weight(1f)
        )
        BottomNavigationContent(
            homeScreenState = homeScreenState
        )
    }
}

@Composable
fun HomeScreenContent(
    homeScreen: BottomNavType,
    modifier: Modifier
) {
    Column(modifier = modifier) {
        Crossfade(homeScreen) { screen ->
            Surface(color = MaterialTheme.colorScheme.background) {
                when (screen) {
                    BottomNavType.HOME -> HomeScreen()
                    BottomNavType.ABOUT -> AboutScreen()
                }
            }
        }
    }
}

@Composable
fun BottomNavigationContent(
    modifier: Modifier = Modifier,
    homeScreenState: MutableState<BottomNavType>
) {
    var animate by remember { mutableStateOf(false) }
    NavigationBar(
        modifier = modifier,
    ) {
        NavigationBarItem(
            icon = {
                Image(
                    FontAwesome.Icon.faw_home,
                    colorFilter = ColorFilter.tint(MaterialTheme.colorScheme.primary),
                )
            },
            selected = homeScreenState.value == BottomNavType.HOME,
            onClick = {
                homeScreenState.value = BottomNavType.HOME
                animate = false
            },
            label = {
                Text(
                    text = stringResource(id = R.string.navigation_item_home),
                    style = TextStyle(fontSize = 12.sp)
                )
            }
        )
        NavigationBarItem(
            icon = {
                Image(
                    FontAwesome.Icon.faw_info_circle,
                    colorFilter = ColorFilter.tint(MaterialTheme.colorScheme.primary),
                )
            },
            selected = homeScreenState.value == BottomNavType.ABOUT,
            onClick = {
                homeScreenState.value = BottomNavType.ABOUT
                animate = false
            },
            label = {
                Text(
                    text = stringResource(id = R.string.navigation_item_about),
                    style = TextStyle(fontSize = 12.sp)
                )
            }
        )
    }
}

